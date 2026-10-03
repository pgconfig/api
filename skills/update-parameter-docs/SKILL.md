---
name: update-parameter-docs
description: Regenerate the PostgreSQL parameter documentation in parameters/ from the PostgreSQL source. Use after a PostgreSQL release, when pgconfig adds a major version, or when the extractor stops.
---

# Update parameter docs

`parameters/` is generated: the extractor in `crates/parameter-docs` writes it
from a PostgreSQL git checkout, and nothing else does. When PostgreSQL's
sources outgrow the extractor, change the extractor. `parameters/README.md`
describes the files, and ADR 0003 the decision.

## Steps

1. Find a PostgreSQL clone in `$PG_CHECKOUT_DIR`, or clone
   `https://github.com/postgres/postgres.git`. Fetch its tags:
   `git -C "$PG_CHECKOUT_DIR" fetch --tags`. The extractor reads the files at
   each tag from git, so the clone's working tree stays as it is.
2. Run `cargo run -p pgconfig-parameter-docs -- --postgres "$PG_CHECKOUT_DIR"`,
   optionally followed by the major versions to extract. Done when it prints
   one line per version and exits 0.
3. When it stops, fix the cause from the table below and run it again.
4. Review the diff. `parameters/sources.yml` shows the new release tags. Read
   every changed front matter line: each new or removed parameter, and each
   changed default, unit, limit, or enum value, must match the PostgreSQL
   release notes. A minor release touches few files; many changed files that
   the release notes do not explain mean an extractor bug.
5. For each changed version with an official Docker image (9.5 and later),
   check the settings against `pg_settings` as below.
6. Run `just test`, add a patch changeset, and commit with `my-commit`. The
   answers of MCP and `/parameters` change, so the change is user-visible.

## When the extractor stops

| Message | Change |
| --- | --- |
| `<element> is not rendered` | Render the element in `src/manual.rs`, with a test in `tests/manual.rs` |
| `the entity &x; is not known` | Add the entity to `entity` in `src/markup.rs` |
| `the preprocessor condition X is not in the platform table` | Add X to `src/platform.rs` |
| `X is not defined`, or `X has several definitions` | Add the Linux definition of X to `src/platform.rs` |

The platform table describes a standard x86-64 Linux build, like the PGDG
packages and the official Docker images: OpenSSL, LZ4, Zstandard, liburing,
and no assertions or debugging aids. Look up how PostgreSQL's headers and
configure define each macro at the tag, and write the reason as a comment
beside the entry.

## Check against pg_settings

```sh
docker run --platform linux/amd64 -d --name pgsettings -e POSTGRES_HOST_AUTH_METHOD=trust postgres:18
docker exec pgsettings psql -U postgres -Atc "select json_agg(s) from (select name, unit, boot_val, min_val, max_val, context, category, enumvals from pg_settings) s"
docker rm -f pgsettings
```

Compare each parameter of `parameters/18/` with its row: `default` with
`boot_val`, `min` with `min_val`, `max` with `max_val`, `values` with
`enumvals`, and `unit`, `context`, and `category` as named. Three differences
are expected: `krb_server_keyfile` and `unix_socket_directories` carry the
Debian paths, and `server_version` the Debian suffix. Any other difference is a
platform table entry to fix.

## A new major version

The extractor covers the versions in `SUPPORTED` in
`crates/parameter-docs/src/lib.rs`, which follows `PgMajor::supported()` in
`crates/pgconfig/src/version.rs`. Add the version to both, then extract it.
The extractor reads C declarations through 18 and `guc_parameters.dat` from
19. It chooses a final release first, then the newest RC or beta when no
final release exists. Verify the tag in `parameters/sources.yml`.

For 19 beta, validate with `postgres:19beta4` rather than `postgres:19`.
The CI image digest is pinned in `.github/workflows/integration.yml`; update
it together with the source tag when moving to another prerelease. Keep 18
as the product default until a separate change is approved.
