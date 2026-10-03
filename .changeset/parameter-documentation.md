---
"pgconfig-api": minor
---

pgconfig now ships the PostgreSQL manual's entry for every parameter of every
supported version, from 9.1 to 18, with its type, context, unit, default,
limits, and accepted values.

- MCP has two new tools: `list_postgres_parameters` finds parameters by version,
  category, or text, and `describe_postgres_parameter` returns one parameter's
  entry. See `docs/mcp.md`.
- The server answers `GET /parameters/<version>/<name>.md` with the entry as
  Markdown.
- The web app shows the manual's text for each parameter of the comparison.
