# Go to Rust migration assessment

Checked on 2026-09-30 against this repository at `507fdd3`, `pgconfig/web`,
`pgconfig/docs`, and the Momoi Labs repositories `blueprint`, `self-host`, and
`pg-probe`.

## Bottom line

Rewrite in Rust. The reason is strategic fit. Rust gives no runtime advantage
for this workload.

- The code is small: 2,591 lines of Go outside tests. The engine is pure and
  deterministic, so parity is cheap to prove with golden files.
- The core already needs a redesign. Epic #43 specifies it. Open PRs #51 to #55
  implement part of it in Go and none is merged. Finishing that redesign in Go
  and then porting it means paying twice.
- The products that will build on the engine are Rust (`pg-probe` today).
- Kiso is React. The Vue frontend must be rewritten to adopt it, whatever the
  backend language is. That is the largest piece of work, and Rust does not
  change it.
- The crate serves your own products. Outside users integrate over HTTP and
  the CLI, so REST v1 and `pgconfigctl` stay the public contract.

The recommendation flips if the monitoring product is dropped or is not built
in Rust. In that case, keep Go and only do the frontend and monorepo work.

## What exists today

| Part | Stack | Size | Source |
| --- | --- | --- | --- |
| Engine | Go, `pkg/` | 1,662 lines | `pkg/rules`, `pkg/category`, `pkg/format`, `pkg/input` |
| REST API | Fiber v2, swaggo | 503 lines | `cmd/api` |
| CLI | Cobra | 296 lines | `cmd/pgconfigctl` |
| Docs scraper | goquery | 130 lines | `generators/pg-docs` |
| Tests | testify, goconvey | 1,275 lines | `*_test.go` |
| Web | Vue 3, shadcn-vue, Vite 6 | about 3,300 lines without vendored UI | [pgconfig/web](https://github.com/pgconfig/web) |
| Docs | VitePress | 7 Markdown pages | [pgconfig/docs](https://github.com/pgconfig/docs) |

History: 181 commits since 2020-08-26, 169 of them by one author. A GitHub
code search for `github.com/pgconfig/api/pkg` found no importer outside the
`pgconfig` org, so dropping the Go module should break nobody. Code search does
not index every repository, so treat this as "none found".

## Why the core needs a redesign anyway

Epic [#43](https://github.com/pgconfig/api/issues/43) and issues #44 to #50
define a "provenance-aware tuning boundary": Tuning Request, Tuning
Recommendation, Tuning Assumption, warnings, and a reason per value. All eight
issues are open. PRs #51 to #55 cover #44, #45, #48, #49, and #50 in Go and are
unmerged. #46 and #47 have no PR. The issues exist because the current engine
has these problems:

- PostgreSQL versions are `float32` (`pkg/input/input.go`). `17.10` becomes
  `17.1`, and version checks are float comparisons.
- OS matching is inconsistent. `ValidOS` lowercases its input, but the Windows
  rules compare the raw string (`pkg/rules/os.go`). `Windows` passes validation
  and then skips the Windows rules.
- An unknown disk type gets the HDD value for `effective_io_concurrency` and
  the SSD value for `random_page_cost` (`pkg/rules/storage.go`).
- Output is built with reflection over struct tags
  (`pkg/category/export_slice.go`), and "zero" means "parameter removed"
  (`pkg/rules/version.go`).
- `pkg/category/checkpoint.go` recomputes `shared_buffers` with a copy of the
  table from `pkg/category/memory.go`.

Rust enums and `Option` model most of this directly. The issue #44 spec can be
used as the public API of the crate with almost no change.

## Fit with the Momoi Labs standard

| Concern | Momoi Labs practice | Source |
| --- | --- | --- |
| HTTP | axum 0.8 | `self-host/Cargo.toml`, `pg-probe/Cargo.toml` |
| CLI | clap 4 | same |
| UI | React 19 with `@momoi-labs/kiso-react`, built with Vite | `self-host/console/package.json` |
| UI delivery | Static assets embedded in the Rust binary. Node is a build tool only | `pg-probe/docs/adr/0007-embedded-web-ui-via-pg-probe-analyze.md` |
| API style | HTTP JSON | `self-host/docs/adr/0007-operator-api-http-json.md` |
| Releases | GoReleaser with the Rust builder | `pg-probe/README.md` |

Two consequences:

1. Kiso ships as `@momoi-labs/kiso` 0.14.0 (tokens and CSS) and
   `@momoi-labs/kiso-react` 0.10.0 (peer dependency `react ^19`), checked on
   npm on 2026-09-30. There is no Vue package. Adopting Kiso means a React
   rewrite of `pgconfig/web`.
2. The standard says no Node runtime. "One site" means one Rust binary that
   serves the API, the web app, and the docs. A separate Node application would
   go against `pg-probe` ADR 0007.

## Rust ecosystem check

Versions read from the crates.io API on 2026-09-30.

| Need | Go today | Rust | Status |
| --- | --- | --- | --- |
| HTTP server | Fiber 2.52 | [axum](https://crates.io/crates/axum) 0.8.9 | Already used at Momoi Labs |
| OpenAPI | swaggo | [utoipa](https://crates.io/crates/utoipa) 6.0.0 | Active, released 2026-09-22 |
| CLI | Cobra | [clap](https://crates.io/crates/clap) 4.6.7 | Already used |
| MCP (epic #43) | not built | [rmcp](https://crates.io/crates/rmcp) 3.5.0, the official SDK | Active, released 2026-09-28 |
| Host RAM and CPU | go-osstat | [sysinfo](https://crates.io/crates/sysinfo) 0.39.6 | Active |
| Embedded assets | none | [rust-embed](https://crates.io/crates/rust-embed) 8.12.0 | Already used |
| YAML | yaml.v2 | [serde_yaml](https://crates.io/crates/serde_yaml) is marked deprecated | Open point, see risks |
| Snapshot tests | none | [insta](https://crates.io/crates/insta) 1.48.0 | Active |

Crate names `pgconfig`, `pgtune`, `pg-tune`, and `pgconfig-core` were all
unregistered on crates.io on 2026-09-30. Searches for "pgtune" and "postgresql
tuning" returned no crate that computes configuration recommendations.

## Interfaces to preserve

| Interface | Contract | How to prove parity |
| --- | --- | --- |
| REST v1 | `GET /v1/tuning/get-config`, `/get-config-all-environments`, `/list-environments`, `/v1/version` (`cmd/api/routes/routes.go`) | Golden responses recorded from the Go binary over an input matrix |
| CLI | `pgconfigctl tune` flags and the `conf`, `alter_system`, `json`, and StackGres formats | Golden stdout per format |
| Generated config | Must load in real PostgreSQL | Keep `.github/workflows/integration.yml`, which runs the output on PostgreSQL 9.5 to 18 |
| Artifacts | Images `pgconfig/api` and `pgconfig/pgconfigctl`, deb, rpm, archives (`.goreleaser.yml`) | Same names from the GoReleaser Rust builder |
| Web | Calls `https://api.pgconfig.org/v1/tuning/` (`web/src/http/index.js`) | Covered by REST v1 parity |

The new server keeps v1 as a thin compatibility layer over the crate. v2 and
MCP expose the richer result.

Do not remove v1. Public installers call it at install time, for example
[kobo-install](https://github.com/kobotoolbox/kobo-install/blob/57215a9eb86a30040aa5ceb702eb3b921e967cb9/helpers/config.py#L1919-L1943)
and the
[Claranet Ansible role](https://github.com/claranet/ansible-role-postgresql/blob/501d30670b498ce30959b62e1fe975b3672413e5/defaults/main.yml#L306-L318).
Mark it deprecated in the docs and keep serving it. Keeping it costs little
because it shares the engine.

## Risks

- **JSON bytes will differ.** Fiber uses `encoding/json`, which escapes `<`,
  `>`, and `&` as `<` style sequences. `serde_json` does not. `rules.yml`
  contains Markdown blockquotes, so `show_doc=true` responses differ in bytes
  while being equal as JSON. Compare parsed JSON in golden tests.
- **Float rounding.** Memory values are computed in `float32` and truncated
  (`pkg/category/memory.go`). Rust `f32` follows the same IEEE rules, but the
  operation order must match. The golden matrix should include odd RAM sizes.
- **YAML crate.** `serde_yaml` is deprecated. `serde_yaml_ng` and
  `serde_norway` had no release in 2025 or 2026. The safest choice is to convert
  `rules.yml` and `pg-docs.yml` at build time and embed the result.
- **Errors in v1.** Every validation error returns HTTP 500 today because the
  handlers return plain errors. Decide whether v1 keeps that or fixes it.
- **Docs scraper.** `generators/pg-docs` scrapes postgresqlco.nf with CSS
  selectors. It runs rarely. Port it last, or leave it in Go until its data
  source is reconsidered.
- **Stall risk.** The MCP epic is the most market-facing work in the backlog.
  A rewrite delays it unless the crate is built first and MCP lands on it next.

## Recommended shape

One repository, one Cargo workspace, one web app:

```
crates/pgconfig/          tuning engine, no I/O, published to crates.io
crates/pgconfig-server/   axum: REST v1, REST v2, MCP, embedded web
crates/pgconfigctl/       clap CLI
web/                      React 19, Vite, Kiso: tuning UI and docs pages
rules.yml, pg-docs.yml    data, embedded at build time
```

Decided on 2026-10-01: transfer `pgconfig/api` to `momoi-labs/pgconfig`. A
transfer keeps its stars, issues, and release history. GitHub redirects the old
URL. `pgconfig/web` and `pgconfig/docs` are archived after their content moves
in.

## Recommended order

1. Record golden outputs from the Go binaries (REST v1 and CLI).
2. Build the `pgconfig` crate to the issue #44 spec, plus a projection that
   reproduces v1 output. Pass the goldens.
3. Ship `pgconfigctl` and the server with REST v1 in Rust. Remove the Go code.
4. Add MCP and REST v2 on the crate (epic #43, retargeted).
5. Rewrite the web app in React with Kiso, fold in the seven docs pages, embed
   both in the server. Archive the old repositories.
6. Mark v1 as deprecated in the docs. Keep serving it while public installers
   call it.

Steps 1 to 3 are small and reversible. Step 5 is the largest and does not
depend on the language decision.
