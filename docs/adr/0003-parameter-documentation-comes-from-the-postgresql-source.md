---
status: accepted
date: 2026-10-03
supersedes: []
superseded_by: null
tags: [documentation, mcp, web]
---

# Parameter documentation comes from the PostgreSQL source

`parameters/` holds the PostgreSQL manual's entry for every parameter of every
supported major version: one Markdown file per parameter, with its settings as
YAML front matter. `crates/parameter-docs` extracts it from a PostgreSQL git
checkout at the newest release tag of each version. The text comes from
`doc/src/sgml/config.sgml`, and the context, unit, default, and limits come
from the GUC tables in C. It replaces the Go scraper of postgresqlco.nf.

> **Append-only:** never edit an accepted ADR. To change a decision, write a
> new ADR and link it to the old one via `supersedes` / `superseded_by`.

## Considered options

- **Keep scraping postgresqlco.nf.** Rejected: it depends on a third party's
  page markup, it covered 22 parameters, and its recommendation texts are not
  the PostgreSQL manual.
- **The HTML manual on postgresql.org, with `pg_settings` from Docker.**
  Rejected: a page changes under the same address, and the images of 9.1 to
  9.4 no longer run, so those versions would have no limits.
- **The PostgreSQL source at its release tags.** Chosen: a tag gives the same
  files every time, offline, for every supported version.

## Consequences

- `pg-docs.yml` stays as REST v1 data. The goldens pin what `show_doc=true`
  returns, so it is never regenerated and nothing else reads it.
- The settings are those of a standard 64-bit Linux build.
  `crates/parameter-docs/src/platform.rs` records every preprocessor decision,
  and the extractor stops on a macro it does not list. For 9.5 to 18 the
  values were checked against `pg_settings` of the official Docker images.
- MCP gains `list_postgres_parameters` and `describe_postgres_parameter`. The
  server answers `GET /parameters/<major>/<name>.md` with the file as it is,
  and the web app reads it for the parameter detail. `/parameters` joins the
  server paths of ADR 0002.
- The text belongs to the PostgreSQL Global Development Group and is under the
  PostgreSQL License. `parameters/COPYRIGHT` keeps its notice.
- The `update-parameter-docs` skill runs the extraction for a new release.
