# /// script
# requires-python = ">=3.10"
# dependencies = ["mido>=1.3,<2", "python-rtmidi>=1.5,<2"]
# ///
"""Manual Rekordbox EQ experiment. No application state is inferred."""
import argparse
import json
import math
from pathlib import Path
import time


def messages(mapping, value):
    channel = mapping["channel"]
    if type(channel) is not int or not 1 <= channel <= 16:
        raise ValueError("channel must be an integer from 1 to 16")
    encoding = mapping["encoding"]
    keys = ["cc"] if encoding == "cc7" else ["msb_cc", "lsb_cc"]
    if encoding not in ("cc7", "cc14"):
        raise ValueError("encoding must be cc7 or cc14")
    for key in keys:
        if type(mapping[key]) is not int or not 0 <= mapping[key] <= 127:
            raise ValueError(f"{key} must be an integer from 0 to 127")
    maximum = 127 if encoding == "cc7" else 16383
    if type(value) is not int or not 0 <= value <= maximum:
        raise ValueError(f"value must be an integer from 0 to {maximum}")
    status = 0xB0 + channel - 1
    if encoding == "cc7":
        return [[status, mapping["cc"], value]]
    if mapping["msb_cc"] == mapping["lsb_cc"]:
        raise ValueError("MSB and LSB controllers must differ")
    pair = [[status, mapping["msb_cc"], value >> 7],
            [status, mapping["lsb_cc"], value & 127]]
    order = mapping.get("order", "msb-first")
    if order not in ("msb-first", "lsb-first"):
        raise ValueError("order must be msb-first or lsb-first")
    return pair if order == "msb-first" else pair[::-1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["ports", "monitor", "send"])
    parser.add_argument("--port", help="Exact endpoint name")
    parser.add_argument("--mapping", type=Path)
    parser.add_argument("--value", type=int)
    parser.add_argument("--seconds", type=float, default=30)
    parser.add_argument("--log", type=Path)
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--safe-level-confirmed", action="store_true")
    args = parser.parse_args()
    if not math.isfinite(args.seconds) or args.seconds <= 0:
        parser.error("seconds must be finite and positive")
    raw = None
    if args.action == "send":
        if args.mapping is None or args.value is None:
            parser.error("send requires --mapping and --value")
        try:
            raw = messages(json.loads(args.mapping.read_text()), args.value)
        except (ValueError, KeyError, TypeError, OSError) as exc:
            parser.error(str(exc))
        if not args.execute:
            print(json.dumps({"event": "dry_run", "bytes": raw}))
            return
        if not args.safe_level_confirmed:
            parser.error("set a safe monitoring level and supply --safe-level-confirmed")
    if args.action != "ports" and (not args.port or not args.log):
        parser.error("live operations require --port and --log")
    import mido
    if args.action == "ports":
        print(json.dumps({"inputs": mido.get_input_names(),
                          "outputs": mido.get_output_names()}, indent=2))
        return
    names = mido.get_input_names() if args.action == "monitor" else mido.get_output_names()
    if names.count(args.port) != 1:
        parser.error("port must match exactly one endpoint from ports")
    # Exclusive creation prevents accidental loss of earlier evidence.
    with args.log.open("x") as log:
        def record(event, **fields):
            row = {"event": event, "monotonic_ns": time.monotonic_ns(),
                   "wall_time_ns": time.time_ns(), **fields}
            log.write(json.dumps(row) + "\n")
            log.flush()
        record("session", action=args.action, port=args.port,
               mapping=json.loads(args.mapping.read_text()) if raw else None)
        try:
            if args.action == "send":
                with mido.open_output(args.port, autoreset=False) as port:
                    for data in raw:
                        record("send_attempt", bytes=data, requested_position=args.value)
                        port.send(mido.Message.from_bytes(data))
                        record("transport_accepted", bytes=data)
            else:
                with mido.open_input(args.port) as port:
                    end = time.monotonic() + args.seconds
                    while time.monotonic() < end:
                        for message in port.iter_pending():
                            record("received", bytes=message.bytes())
                        time.sleep(0.001)
        except KeyboardInterrupt:
            record("interrupted")
        except Exception as exc:
            record("error", detail=str(exc))
            raise
        finally:
            record("session_end")


if __name__ == "__main__":
    main()
