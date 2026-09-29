import unittest
from probe import messages


class EncodingTests(unittest.TestCase):
    def setUp(self):
        self.mapping = dict(channel=1, encoding="cc14", msb_cc=15, lsb_cc=47)

    def test_positions(self):
        for value, msb, lsb in [(0, 0, 0), (8192, 64, 0), (8193, 64, 1), (16383, 127, 127)]:
            self.assertEqual(messages(self.mapping, value), [[176, 15, msb], [176, 47, lsb]])

    def test_order_and_channel(self):
        self.mapping.update(channel=2, order="lsb-first")
        self.assertEqual(messages(self.mapping, 8193), [[177, 47, 1], [177, 15, 64]])

    def test_cc7(self):
        self.assertEqual(messages(dict(channel=16, encoding="cc7", cc=5), 64), [[191, 5, 64]])

    def test_invalid(self):
        for value in [-1, 16384, True, 0.5]:
            with self.assertRaises(ValueError):
                messages(self.mapping, value)
        for patch in [dict(channel=0), dict(msb_cc=128), dict(order="other"), dict(encoding="other")]:
            with self.assertRaises(ValueError):
                messages(self.mapping | patch, 0)


if __name__ == "__main__":
    unittest.main()
