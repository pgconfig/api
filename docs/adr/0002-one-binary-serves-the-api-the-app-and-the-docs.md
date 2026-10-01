---
status: accepted
date: 2026-10-01
supersedes: []
superseded_by: null
tags: [architecture, web]
---

# One binary serves the API, the web app, and the docs

`pgconfig-server` embeds the built web app and serves it next to REST v1, the
OpenAPI document, and MCP. There is no Node runtime: `web/` is built by Vite
before cargo, and `web/dist` is not committed. This follows `self-host`
ADR-0016 and `pg-probe` ADR 0007.

> **Append-only:** never edit an accepted ADR. To change a decision, write a
> new ADR and link it to the old one via `supersedes` / `superseded_by`.

## Consequences

- The paths are split by prefix. `/v1` is REST v1, `/docs` is the Swagger UI
  with the OpenAPI document at `/docs/openapi.json`, and `/mcp` is MCP. Every
  other read falls back to the web app, which routes in the browser.
- `/docs` was already the Swagger UI, so the pages that came from
  `pgconfig/docs` live under `/guide`. A new server route must not take a path
  the app uses.
- The web app calls the API on its own origin. The Vue app called
  `api.pgconfig.org`, so serving the app and the API on different hosts needs
  `VITE_API_BASE_URL` at build time.
- The web build is a step before cargo, in CI and in the release. The server
  compiles without the bundle and then serves the API only.
- `/mcp` has a stricter origin policy than `/v1`, which answers any origin as
  it always did. `docs/mcp.md` has the details.
