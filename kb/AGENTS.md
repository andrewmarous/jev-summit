# Knowledge base schema

Use this schema when ingesting sources into `kb/`, answering from the wiki, or maintaining wiki pages.

## Layers

- `raw/`: curated, immutable sources. Read them but never edit them. Add new files for new evidence, including dated conversation notes or test records.
- `wiki/`: LLM-maintained Markdown synthesis. Update existing pages as evidence changes. Store plans and result templates here.
- `AGENTS.md`: this schema. Update it with the user when the workflow needs to change.

## Ingest

1. Read the new raw source and identify claims, context, dates, and uncertainties.
2. Create or update a source note in `wiki/sources/` with a link to the raw file. Finish when each important claim has a traceable source.
3. Update affected topic pages and cross-links. Label proposed designs as hypotheses and test results as observations.
4. Update `wiki/index.md` with links and one-line summaries of new pages.
5. Append a dated `ingest` entry to `wiki/log.md` with the changed pages and source. Finish when the index and log match the edits.

## Query

1. Read `wiki/index.md`, then relevant pages and their raw sources when accuracy depends on details.
2. Answer with links to wiki pages and raw evidence. Distinguish an observed fact from a proposal.
3. If the answer yields reusable knowledge, update the relevant page or add a new page. Update the index and append a `query` entry to the log.

## Lint

Inspect links, orphan pages, contradictory claims, stale assumptions, unsupported observations, and gaps. Reconcile wiki pages with raw evidence. Append a `lint` entry to the log with changes and remaining questions.

## Conventions

Use relative Markdown links. Keep source notes separate from synthesis pages. Record the date, machine, Rekordbox version, mode, hardware, and routing with every probe result. Cite a raw source or a dated probe record for every claim marked **Observed**. Keep the log append-only. The index is a catalog, not a timeline.
