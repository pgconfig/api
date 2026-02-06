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

## Adding a New Rule

1. Create function in `pkg/rules/` with signature `func(*input.Input, *category.ExportCfg) (*category.ExportCfg, error)`
2. Add to `allRules` slice in `pkg/rules/compute.go` (mind order)
3. Write unit tests
4. Update `rules.yml` if rule needs metadata

## CI/CD

- **cover.yml**: runs `make build` and `make test`, pushes coverage to Coveralls
- **release.yml**: triggered on tags, runs goreleaser for multi‑arch binaries and Docker images

## Commit Conventions

Follow commit conventions from `~/.claude/pgconfig.md`:

```
<type>: <subject line (max 50 chars)>

<body wrapped at 80 cols, focus on WHY not WHAT>
```

Types: `feat`, `fix`, `refactor`, `docs`, `chore`, `test`, `style`

Rules:
- Title ≤50 chars, imperative mood ("fix" not "fixed").
- Body wrapped at 80 cols, focus on WHY.
- Sign‑off required (`-s`).
- **STRICTLY FORBIDDEN**: AI attribution footers (e.g., "Generated with Crush", "Assisted by...").
- **STRICTLY FORBIDDEN**: Adding "Co-authored-by" unless explicitly requested by the user.
- **English Only**: Commit messages must be in English.

## Gotchas

1. **Swagger docs before building**: `make build` depends on `make docs`
2. **Byte parsing**: case‑insensitive, expects unit (KB, MB, GB, TB)
3. **PostgreSQL version defaults**: default is 18, supported 9.1–18
4. **Rule order**: `computeVersion` must be last (removes unsupported parameters)
5. **AIO parameters (PostgreSQL 18+)**: `io_method` and `io_workers` only available in ≥18. `io_workers` scaled by profile: Desktop 10%, WEB 20%, Mixed 25%, OLTP 30%, DW 40%, +10% for HDD.
