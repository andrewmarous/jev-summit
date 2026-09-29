# Karpathy: LLM Wiki

Source: [original gist snapshot](../../raw/karpathy-llm-wiki.md). [Online original](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f).

The design uses three layers: immutable curated raw sources, an LLM-maintained interlinked Markdown wiki, and a schema that defines maintenance workflows. Ingest incorporates new evidence into existing pages. Query starts with an index and can file useful synthesis back into the wiki. Lint looks for contradictions, stale claims, orphan pages, and missing links. `index.md` catalogs pages; `log.md` records chronological changes. The gist leaves the specific folder layout to the project.
