# Agent Guide for pgconfig/api

Essential commands, structure, and patterns for AI agents.

## Essential Commands

```bash
make docs         # Generate Swagger API documentation
make test         # Run all tests with race detector and coverage
make lint         # Run go vet (requires docs generated first)
make build        # Clean, generate docs, lint, and build binaries
make clean        # Remove dist/ and generated docs
```

## Project Structure

```
.
├── cmd/                 # API and CLI entry points
├── pkg/                 # Core packages (input, rules, category, format, docs)
├── generators/pg-docs/  # Tool to generate pg-docs.yml
├── rules.yml            # Rule metadata (categories, abstracts, recommendations)
└── pg-docs.yml          # PostgreSQL parameter documentation per version
```

## Code Patterns

- **Go 1.25.1**, module `github.com/pgconfig/api`
- **Fiber** for API, **Cobra** for CLI, **Swagger** for docs
- **English Language**: All code comments, documentation, and variable names must be in English.
- Input parsing: `pkg/input/bytes.Parse()` for byte units, `profile.Profile` for workload types
- Rule pipeline in `pkg/rules/compute.go` (order: arch → OS → profile → storage → AIO → version)
- Three output formats: `json`, `alter_system`, `conf`
- Configuration files: `rules.yml` and `pg-docs.yml` loaded at startup

## Testing

- `make test` runs all tests with coverage (generates `covprofile`)
- Test files follow `*_test.go` pattern
- CI runs tests on push/pull request (`.github/workflows/cover.yml`)
- `tests/golden/` pins the output of REST v1 and `pgconfigctl`. A change to the
  rules must come with re-recorded goldens. See `tests/golden/README.md`.

## Rust workspace

The Rust rewrite lives next to the Go code until the cutover. See
`docs/research/rust-migration-assessment.md`.

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

- `crates/pgconfig`: the tuning engine, no I/O. `tune` is the entry point.
  The `v1` module reproduces the output of REST v1 and `pgconfigctl`.
- `crates/pgconfigctl`: the CLI (clap). Same flags and output as the Go CLI.
- `crates/pgconfig-server`: the HTTP server (axum). REST v1, the OpenAPI
  document under `/docs`, and the MCP endpoint at `/mcp`. `docs/mcp.md` is the
  MCP contract: change it with the code.
- `crates/golden`: records and replays `tests/golden`. `cargo test` replays
  every golden against the Rust binaries.
- `scripts/check-conf-loads.sh <pgconfigctl> <version>` loads a generated
  config in a real PostgreSQL container.
- The toolchain is pinned in `mise.toml`. Run cargo through `mise exec --` when
  it is not on the `PATH`.
- `rules.yml` and `pg-docs.yml` are compiled into the crate by its `build.rs`.

## Adding a New Rule

1. Create function in `pkg/rules/` with signature `func(*input.Input, *category.ExportCfg) (*category.ExportCfg, error)`
2. Add to `allRules` slice in `pkg/rules/compute.go` (mind order)
3. Write unit tests
4. Update `rules.yml` if rule needs metadata

## CI/CD

- **cover.yml**: runs `make build`, `make test`, and the release config checks
  in parallel
- **changesets.yml**: on `main`, opens or updates the Changesets version PR;
  merging it tags `v<version>` and calls `release.yml`
- **release.yml**: publishes a tag with GoReleaser for multi-arch binaries and
  Docker images, using the tag's `CHANGELOG.md` entry as release notes
- **pr-title.yml**: validates pull request titles as Conventional Commits

## Commit Conventions

Follow commit conventions from `~/.claude/pgconfig.md`:

```
<type>: <subject line (max 50 chars)>

<body wrapped at 80 cols, focus on WHY not WHAT>
```

Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `style`, `ci`,
`build`, `perf`, `revert`

Rules:
- Title ≤50 chars, imperative mood ("fix" not "fixed").
- Body wrapped at 80 cols, focus on WHY.
- Use `feat` only for user-facing product capabilities. Release automation,
  workflows, and other CI/CD infrastructure must use `ci`.
- Add a changeset (`npm run changeset`) for each user-visible change and commit
  it with the change. CI and docs-only changes need none. See
  `docs/releases.md`.
- Sign‑off required (`-s`).
- **STRICTLY FORBIDDEN**: AI attribution footers (e.g., "Generated with Crush", "Assisted by...").
- **STRICTLY FORBIDDEN**: Adding "Co-authored-by" unless explicitly requested by the user.
- **English Only**: Commit messages must be in English.
- **Breaking changes require explicit approval**: Agents must never choose a
  `major` changeset, add `!` to a commit/PR type, or add a `BREAKING CHANGE:`
  footer unless the user explicitly requests a breaking change or major release.
  A large change is not necessarily a breaking change; when in doubt, use a
  non-breaking type and ask the user.

## Gotchas

1. **Swagger docs before building**: `make build` depends on `make docs`
2. **Byte parsing**: case‑insensitive, expects unit (KB, MB, GB, TB)
3. **PostgreSQL version defaults**: default is 18, supported 9.1–18
4. **Rule order**: `computeVersion` must be last (removes unsupported parameters)
5. **AIO parameters (PostgreSQL 18+)**: `io_method` and `io_workers` only available in ≥18. `io_workers` scaled by profile: Desktop 10%, WEB 20%, Mixed 25%, OLTP 30%, DW 40%, +10% for HDD.

## Agent skills

### Issue tracker

Issues and specs for this repository live in GitHub Issues. See
`docs/agents/issue-tracker.md`.

### Triage labels

This repository uses the default triage labels. See
`docs/agents/triage-labels.md`.

### Domain docs

This is a single-context repository. See `docs/agents/domain.md`.

### Commit convention

Create commits with `my-commit` (conventional commits). PR titles and commit
messages are validated by the `conventional-commits` workflow; merges use
rebase-and-merge.
