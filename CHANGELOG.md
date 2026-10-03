# pgconfig-api

## 4.1.0

### Minor Changes

- c060b77: pgconfig now ships the PostgreSQL manual's entry for every parameter of every
  supported version, from 9.1 to 18, with its type, context, unit, default,
  limits, and accepted values.
  
  - MCP has two new tools: `list_postgres_parameters` finds parameters by version,
    category, or text, and `describe_postgres_parameter` returns one parameter's
    entry. See `docs/mcp.md`.
  - The server answers `GET /parameters/<version>/<name>.md` with the entry as
    Markdown.
  - The web app shows the manual's text for each parameter of the comparison.

## 4.0.0

### Major Changes

- b13680e: pgconfig is rewritten in Rust, and one binary now serves the API, the web app,
  and the documentation.
  
  REST v1 and `pgconfigctl` keep their behavior. The routes, parameters,
  defaults, error texts, flags, formats, and output are the same, checked against
  6,121 responses recorded from the last Go release. The deb, rpm, and archive
  names are unchanged.
  
  New:
  
  - An MCP endpoint at `/mcp` with one tool,
    `recommend_postgres_configuration`. Each recommendation comes with the
    reason for its value, the defaults assumed, and warnings. See `docs/mcp.md`.
  - The web app and the guide are part of the server: the profile comparison at
    `/`, the guide at `/guide`.
  
  Breaking:
  
  - The server binary is `pgconfig-server`, and was `api`. The deb and rpm
    packages install it as `/usr/bin/pgconfig-server`.
  - `--rules-file` and `--docs-file` are gone. The rules and the parameter
    documentation are built into the binaries.
  - The Docker images moved to `ghcr.io/momoi-labs/pgconfig` and
    `ghcr.io/momoi-labs/pgconfigctl`. `pgconfig/api` and `pgconfig/pgconfigctl`,
    on Docker Hub and on `ghcr.io/pgconfig`, stop at 3.6.1.
  - `/docs` serves an OpenAPI 3.1 document at `/docs/openapi.json`. It was
    Swagger 2.0 at `/docs/doc.json`.
  - The Go module `github.com/pgconfig/api` no longer exists.
  
  On macOS, `pgconfigctl` reads total memory from the kernel, so the default
  `--ram` is slightly larger than before. Linux is unchanged.
