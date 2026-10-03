# PostgreSQL 19 readiness review

Offer PostgreSQL 19 as a beta option and keep PostgreSQL 18 as the default.
The implementation uses native dynamic I/O pool defaults. Autovacuum tuning
remains a separate decision.

- PostgreSQL 19 removes `io_workers`. Reusing the current PostgreSQL 18 output
  would produce an invalid setting.
- PostgreSQL 19 separates automatic analyze logging from vacuum logging.
  Preserving the pgBadger preset's coverage requires a new setting.
- The other existing tuning parameters need no documented PostgreSQL 19
  formula change. New autovacuum controls deserve evaluation, not invented
  CPU or RAM formulas.

Research checked on 2026-10-03. The user subsequently authorized the initial
support scope below. No performance benchmarks were performed.

## Implementation decisions after the documentation merge

The branch was rebased onto `c060b77`, which introduced the source-based
parameter catalog in [ADR 0003](../adr/0003-parameter-documentation-comes-from-the-postgresql-source.md).
This supersedes the old Go generator assessment below.

- Offer `19 (Beta)` and keep default 18. Core and MCP still require a version.
- Emit `io_min_workers=2` and `io_max_workers=8` for 19. Keep the native timers
  and the existing `io_max_concurrency` values. Preserve all earlier-version
  calculations.
- Add `log_autoanalyze_min_duration=0` only to the 19 pgBadger preset.
- Cap the 19 Windows `io_max_combine_limit` at 16 blocks. Keep Linux DW at
  128 blocks. This resolves the new version's generated configuration without
  changing the frozen 18 output.
- Generate the complete 19 catalog from `REL_19_BETA4`, adapting the Rust
  extractor to `guc_parameters.dat`. Web details, MCP, and `/parameters/19/`
  use this catalog. The legacy `pg-docs.yml` stays frozen, so REST v1
  `show_doc` has no 19 manual metadata. Hide unavailable co.nf links for 19.
- Keep the existing categories and profiles. Autovacuum sizing needs workload
  inputs and measurements before it becomes a recommendation.

The inventory and proposal below record the evidence behind these choices.

Decision sections: [parameters and categories](#proposed-parameter-and-category-decisions),
[initial scope](#proposed-initial-support-boundary),
[repository changes](#repository-integration-findings).

## Release and evidence boundary

The current prerelease is PostgreSQL 19 beta 4, released on 2026-09-24.
The project expects a release candidate in early October, subject to testing.
Beta 4 also removed previously announced SQL/PGQ support, so early beta
articles are not a reliable inventory. [Official beta 4 announcement](https://www.postgresql.org/about/news/postgresql-19-beta-4-released-3386/)

The source comparison uses release tags, not the development branch:

| Baseline | Tag | Commit |
| --- | --- | --- |
| PostgreSQL 18.6 | `REL_18_6` | [`724edf9bde9d356724ad384a2e196edc3c9f80f7`](https://github.com/postgres/postgres/commit/724edf9bde9d356724ad384a2e196edc3c9f80f7) |
| PostgreSQL 19 beta 4 | `REL_19_BETA4` | [`b73d13c32c834a2c8e1c60cb92f79530376cedf1`](https://github.com/postgres/postgres/commit/b73d13c32c834a2c8e1c60cb92f79530376cedf1) |

Method: compare built-in GUC names, declarations, defaults, contexts, and
configuration documentation. PostgreSQL 18 stores declarations in
[`guc_tables.c`](https://github.com/postgres/postgres/blob/REL_18_6/src/backend/utils/misc/guc_tables.c);
PostgreSQL 19 moves them to
[`guc_parameters.dat`](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/backend/utils/misc/guc_parameters.dat).
The declaration inventory has 418 names in 18.6 and 438 in 19 beta 4:
22 additions and two removals. Conditional build settings are included.
This is exhaustive for those core declarations, not extension GUCs or
table/subscription options. A compiled server's `pg_settings` can differ with
build options.

The configuration text comparison uses the tagged
[18.6 manual source](https://github.com/postgres/postgres/blob/REL_18_6/doc/src/sgml/config.sgml)
and [19 beta 4 manual source](https://github.com/postgres/postgres/blob/REL_19_BETA4/doc/src/sgml/config.sgml).
Readable links below point to the versioned online manual, which can receive
later corrections. Recheck the release tag before implementing or claiming
support for the final release. Avoid `/docs/devel/` and `master` for this review.

## Existing tuning parameters

The inventory below covers every parameter in `Computed::groups`, including
the old-version-only `checkpoint_segments`. REST v1 emits 21 of these for
PostgreSQL 18; the richer `tune` result omits `listen_addresses` and emits 20.
These are compatibility findings, not evidence that each existing heuristic
is optimal for PostgreSQL 19.

The retained parameters keep their declared types, defaults, and bounds in
the source comparison. Documentation for `max_worker_processes` adds a
reference to parallel autovacuum; `io_workers` disappears. Sources:
[18.6 declarations](https://github.com/postgres/postgres/blob/REL_18_6/src/backend/utils/misc/guc_tables.c),
[19 beta 4 declarations](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/backend/utils/misc/guc_parameters.dat),
[19 resource settings](https://www.postgresql.org/docs/19/runtime-config-resource.html),
[19 WAL settings](https://www.postgresql.org/docs/19/runtime-config-wal.html),
[19 planner settings](https://www.postgresql.org/docs/19/runtime-config-query.html).

| Parameter | PostgreSQL 19 finding and proposed treatment |
| --- | --- |
| `shared_buffers` | Retained. Keep the existing baseline; no new percentage is justified by this review. |
| `effective_cache_size` | Retained. Still a planner estimate, not allocated memory. |
| `work_mem` | Retained. It remains a per-operation limit; concurrent and parallel operations can multiply memory use. |
| `maintenance_work_mem` | Retained. Existing warnings about concurrent autovacuum memory still apply. |
| `min_wal_size` | Retained. Keep the existing profile values initially. |
| `max_wal_size` | Retained. No new universal sizing rule follows from the release notes. |
| `checkpoint_completion_target` | Retained, default `0.9`. Current output matches that default. |
| `wal_buffers` | Retained. Automatic `-1` remains available. |
| `checkpoint_segments` | Already absent in both 18 and 19. Keep it restricted to the old supported versions. |
| `listen_addresses` | Retained. No 19-specific change to the v1 output. |
| `max_connections` | Retained. Continue using the request value. |
| `random_page_cost` | Retained. Storage/profile values remain project heuristics. |
| `effective_io_concurrency` | Retained. Default `16`; controls concurrent I/O beyond bitmap scans. Existing SSD/HDD/SAN values are not validated by this research. |
| `maintenance_io_concurrency` | Retained. Default `16`; separate maintenance control. |
| `io_method` | Retained, default `worker`. Do not select `io_uring` solely from the OS name; the build must support it. |
| `io_workers` | Removed. Keep it only for 18. Decide how to represent the new pool settings for 19. |
| `io_max_combine_limit` | Retained. It caps `io_combine_limit`; raising the cap alone does not raise the active combine size. |
| `io_max_concurrency` | Retained. Default `-1` derives the limit from buffers/processes. Current explicit values override this automatic sizing. |
| `file_copy_method` | Retained, default `copy`. `clone` remains dependent on OS and filesystem support. |
| `max_worker_processes` | Retained. If parallel autovacuum is enabled, include its demand in the shared background-worker budget. |
| `max_parallel_workers_per_gather` | Retained. No new per-query formula follows from 19. |
| `max_parallel_workers` | Retained. Parallel autovacuum would compete with other parallel work for this pool. |

Two existing AIO details need an explicit decision during implementation:

- `io_combine_limit` defaults to 128 kB. Typical maximums are 1 MB on Unix and
  128 kB on Windows. The current DW output sets `io_max_combine_limit=128`
  blocks, or 1 MB with 8 kB blocks, even on Windows. This is an existing
  portability issue to verify, not a new PostgreSQL 19 limit. Raising the
  Unix ceiling without setting `io_combine_limit` also leaves the normal
  combine size at 128 kB. [I/O limits](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-COMBINE-LIMIT)
- `io_max_concurrency=64/128/256` is accepted by the declared range, but
  accepted values are not benchmark evidence. The documented default is
  automatic sizing. Decide separately whether 19 should preserve these
  heuristics or use that default. [Concurrency limit](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-MAX-CONCURRENCY)

## Existing logging presets

This covers all 13 distinct parameters emitted by `pgbadger()` and
`log_options()`, not only the core tuning result.

| Parameter or group | PostgreSQL 19 finding | Proposed treatment |
| --- | --- | --- |
| `log_autovacuum_min_duration` | Now controls automatic vacuum only. It no longer covers automatic analyze. | For the 19 pgBadger preset, add `log_autoanalyze_min_duration=0` alongside the existing vacuum value `0`. |
| `log_lock_waits` | Upstream default changes from `off` to `on`. | Keep the preset's explicit `on`; update default metadata. |
| `logging_collector` | Behavior/default retained. Documentation clarifies that collection does not guarantee durable storage. | Retain the setting; avoid promising lossless durable logs. |
| `log_checkpoints`, `log_connections`, `log_disconnections`, `log_temp_files`, `lc_messages`, `log_min_duration_statement` | No relevant 18.6-to-19 change found in their declarations or parameter documentation. | Retain the existing preset values. |
| `log_destination`, `log_line_prefix`, `syslog_facility`, `syslog_ident` | No relevant 18.6-to-19 change found. | Retain the format presets. Validate the downstream pgBadger version separately if parser compatibility is claimed. |

Sources: [19 logging settings](https://www.postgresql.org/docs/19/runtime-config-logging.html),
[tagged logging documentation](https://github.com/postgres/postgres/blob/REL_19_BETA4/doc/src/sgml/config.sgml).
PostgreSQL accepting the settings does not prove that a particular pgBadger
release parses every new PostgreSQL 19 log record.

## Complete new core GUC inventory

Defaults and contexts below come from the
[19 beta 4 declarations](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/backend/utils/misc/guc_parameters.dat).
`reload` means configuration reload, `start` means server restart, `session`
means user-settable, `superuser` means privileged session setting, and
`read-only` means not a configuration recommendation.

| New parameter | Default | Bounds or choices | Context | Relevance |
| --- | --- | --- | --- | --- |
| `io_min_workers` | `2` | `1` to `32` | reload | Dynamic AIO pool floor. |
| `io_max_workers` | `8` | `1` to `32` | reload | Dynamic AIO pool ceiling. |
| `io_worker_idle_timeout` | `60000 ms` | `0` to `2147483647 ms` | reload | Shrinks idle pool toward its floor. |
| `io_worker_launch_interval` | `100 ms` | `0` to `2147483647 ms` | reload | Limits how quickly the pool grows. |
| `autovacuum_max_parallel_workers` | `0` | `0` to `1024`; also constrained by `max_parallel_workers` | reload | Per-autovacuum-worker index parallelism. Zero disables it. |
| `autovacuum_analyze_score_weight` | `1.0` | `0.0` to `10.0` | reload | Analyze priority component. |
| `autovacuum_freeze_score_weight` | `1.0` | `0.0` to `10.0` | reload | Transaction ID age priority component. |
| `autovacuum_multixact_freeze_score_weight` | `1.0` | `0.0` to `10.0` | reload | Multixact age priority component. |
| `autovacuum_vacuum_insert_score_weight` | `1.0` | `0.0` to `10.0` | reload | Insert-triggered vacuum priority component. |
| `autovacuum_vacuum_score_weight` | `1.0` | `0.0` to `10.0` | reload | Update/delete vacuum priority component. |
| `log_autoanalyze_min_duration` | `600000 ms` | `-1` to `2147483647 ms` | reload | New automatic analyze logging control; `-1` disables, `0` logs all. |
| `enable_eager_aggregate` | `on` | boolean | session | Lets the planner partially aggregate before joins. |
| `min_eager_agg_group_size` | `8.0` | nonnegative floating point | session | Planner threshold for average rows per group. |
| `timing_clock_source` | `auto` | `auto`, `system`, `tsc` when supported | superuser | Timing instrumentation; keep automatic selection. |
| `wal_sender_shutdown_timeout` | `-1 ms` | `-1` to `2147483647 ms` | session | Limits shutdown wait for replication; `-1` waits indefinitely. |
| `max_repack_replication_slots` | `5` | `0` to `MAX_BACKENDS` | start | Capacity for `REPACK` slots; operational choice. |
| `password_expiration_warning_threshold` | `604800 s` | `0` to `2147483647 s` | reload | Warns before password expiry; default seven days. |
| `hosts_file` | Normally `PGDATA/pg_hosts.conf` | file path | start | Server-side SNI hostname configuration. |
| `ssl_sni` | `off` | boolean | reload | Enables server-side SNI. |
| `debug_print_raw_parse` | `off` | boolean | session | Debug logging, not general tuning. |
| `debug_exec_backend` | Build-dependent | boolean | read-only | Reports the server process model. |
| `effective_wal_level` | Runtime value | `minimal`, `replica`, `logical` | read-only | Reports actual WAL level; never emit it into a config. |

The I/O hard cap is defined in
[`proc.h`](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/include/storage/proc.h).
The parallel worker cap is defined in
[`bgworker_internals.h`](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/include/postmaster/bgworker_internals.h).
`MAX_BACKENDS` is a source-level ceiling of 262143, with total process and
resource constraints applied separately, not a useful sizing recommendation.
[Process limit definition](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/include/storage/procnumber.h)

### Removed parameters

| Parameter | PostgreSQL 19 treatment |
| --- | --- |
| `io_workers` | Replaced by the dynamic pool controls above. There is no equivalent one-to-one rename. |
| `escape_string_warning` | Removed with nonstandard ordinary string literals. `standard_conforming_strings` remains queryable but rejects `off`. |

Sources: [19 declarations](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/backend/utils/misc/guc_parameters.dat),
[compatibility settings](https://www.postgresql.org/docs/19/runtime-config-compatible.html),
[release notes](https://www.postgresql.org/docs/19/release-19.html).

## Other defaults and semantics to record

These settings are mostly outside today's generated output. They matter when
reviewing proposed additions and refreshing parameter documentation.

| Setting | Change from 18 to 19 | Recommendation |
| --- | --- | --- |
| `jit` | Default `on` becomes `off`. | Accept the upstream default initially. Do not turn it back on for DW without query measurements. |
| `default_toast_compression` | Default becomes `lz4` when compiled in; otherwise remains `pglz`. | Document the build condition. Do not emit an unavailable method. |
| `max_locks_per_transaction` | Default `64` becomes `128`. | Update documentation. No RAM-only formula is established. |
| `log_lock_waits` | Default `off` becomes `on`. | Update metadata; existing pgBadger output already agrees. |
| `wal_compression` | Default remains `off`. `on` now chooses available `zstd`, then `lz4`, then `pglz`; 18 treated `on` as `pglz`. | If added later, distinguish automatic method selection from an explicit algorithm. |
| `wal_level` | Configured `replica` can now operate at logical level when logical slots exist. `effective_wal_level` reports the actual level. | Explain the distinction; do not configure the read-only reporting setting. |
| `log_min_messages` | Changes from enum to string, with process-specific overrides and a mandatory fallback level. Existing plain levels remain accepted. | Update type metadata if this parameter enters the catalog. |
| `wal_receiver_timeout` | Changes from reload-only to user-settable, including per-subscription/user use. | Keep operational choices outside automatic resource tuning. |
| `max_sync_workers_per_subscription`, `max_logical_replication_workers` | Worker budgeting now includes sequence synchronization. | Account for it only when replication requirements are known. |

Sources: [planner settings](https://www.postgresql.org/docs/19/runtime-config-query.html),
[client defaults](https://www.postgresql.org/docs/19/runtime-config-client.html),
[locks](https://www.postgresql.org/docs/19/runtime-config-locks.html),
[WAL](https://www.postgresql.org/docs/19/runtime-config-wal.html),
[logging](https://www.postgresql.org/docs/19/runtime-config-logging.html),
[replication](https://www.postgresql.org/docs/19/runtime-config-replication.html).

Other configuration text changes include MD5 authentication warnings,
FIPS restrictions on `ssl_groups`, the reserved replication slot name
`pg_conflict_detection`, and recovery transaction ID documentation. These do
not require additions to a hardware-based tuning result.
[Authentication](https://www.postgresql.org/docs/19/runtime-config-connection.html),
[replication](https://www.postgresql.org/docs/19/runtime-config-replication.html),
[recovery targets](https://www.postgresql.org/docs/19/runtime-config-wal.html#RUNTIME-CONFIG-WAL-RECOVERY-TARGET)

## Proposed parameter and category decisions

### Dynamic I/O pool belongs in storage

Recommendation: for initial 19 support, omit `io_workers` and use the native
dynamic pool defaults. The implementation emits
explicit `io_min_workers=2` and `io_max_workers=8`, with version-specific
documentation.
Keep the timer defaults unless measurements justify a change.
Omitting these settings leaves any existing server overrides in effect;
the stated defaults describe a fresh configuration, not a reset operation.

The pool grows and shrinks with demand, so reusing the old CPU percentage as
both minimum and maximum would disable its adaptation. Reusing that number
as the maximum alone also has no evidence from this review. Keep these
settings in the existing storage category.
[I/O documentation](https://www.postgresql.org/docs/19/runtime-config-resource.html#RUNTIME-CONFIG-RESOURCE-IO),
[worker implementation](https://github.com/postgres/postgres/blob/REL_19_BETA4/src/backend/storage/aio/method_worker.c)

### Autovacuum is the strongest new category candidate

Recommendation: evaluate an optional autovacuum/maintenance category
separately from initial version support. Its first coherent set would review
the existing `autovacuum_work_mem`, `autovacuum_max_workers`, and
`autovacuum_worker_slots` together with the new
`autovacuum_max_parallel_workers`. This requires a memory and worker budget,
plus workload facts such as index count, write rate, and maintenance backlog.

Parallel autovacuum is disabled by default. Its new limit applies per
autovacuum worker and only to index vacuum/cleanup phases, not all vacuum
work. The five score weights affect table prioritization. Leave them at
`1.0` until observed backlog gives a reason to bias priorities.
[Autovacuum settings](https://www.postgresql.org/docs/19/runtime-config-vacuum.html),
[parallel worker budget](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-PARALLEL-WORKERS)

Ordinary autovacuum workers have their own slot budget. Their parallel helpers
draw from the parallel/background-worker budget. Treating all these processes
as one pool would make the explanation and sizing wrong.
[Autovacuum worker slots](https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-WORKER-SLOTS),
[parallel autovacuum](https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-MAX-PARALLEL-WORKERS)

### Keep other additions in the documentation first

`enable_eager_aggregate` and `min_eager_agg_group_size` are planner controls,
not grounds for a new workload profile. Retain their upstream defaults.
`timing_clock_source=auto` already selects a supported timing source.
Replication shutdown timeouts, SNI, and repack slot counts depend on
deployment requirements that the current hardware/profile inputs do not
describe. [Planner controls](https://www.postgresql.org/docs/19/runtime-config-query.html),
[timing](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-TIMING-CLOCK-SOURCE),
[replication controls](https://www.postgresql.org/docs/19/runtime-config-replication.html)

A setting category groups parameters; a workload profile selects rules.
Nothing found in PostgreSQL 19 requires a new profile beyond WEB, OLTP, DW,
Mixed, and Desktop. A new autovacuum category would be a separate product
decision, not a prerequisite for listing 19.

## Proposed initial support boundary

1. List `19 (beta)` explicitly. Keep 18 as the web, REST v1, and CLI default.
   The core and MCP currently require an explicit version; keep that contract.
2. Keep existing 18-and-earlier calculations stable. Handle the removed I/O
   setting and automatic analyze logging specifically for 19.
3. Refresh version 19 parameter documentation from a checked, complete source.
   Treat missing metadata as a validation failure, not a successful empty result.
4. Validate generated settings against the pinned 19 beta server, including
   profiles, output formats, and platform-specific limits, before advertising
   compatibility. Benchmarks are a separate requirement for new tuning claims.
5. Recheck the GUC inventory at the release candidate and final release.
   Changing the default from 18 remains a separate decision.

The implementation decisions above settle the dynamic pool presentation.
Autovacuum remains a later, measured change.

## Repository integration findings

Reviewed at commit `0fde6505012a610830c9c63854991e20f5f3bca9`.
This table records the repository before the documentation merge and the
implementation. The decisions above describe the current scope.

| Area | Current behavior | Change needed for 19 |
| --- | --- | --- |
| [Version validation](../../crates/pgconfig/src/version.rs) | `PgMajor::MODERN` ends at 18. Tests explicitly reject 19. | Accept major 19 and update supported-version tests and error text. Keep the numeric input `19`; a UI beta label does not require accepting `19beta4`. |
| [Web version options](../../web/src/lib/options.ts) | The list starts with `18 (Latest)`. | Add `19 (Beta)` and identify 18 as stable/default. Keep [DEFAULT_FORM](../../web/src/lib/formQuery.ts) at 18. |
| [REST v1](../../crates/pgconfig-server/src/v1/mod.rs) and [CLI](../../crates/pgconfigctl/src/main.rs) | Both default to 18. Their legacy float parser already accepts 19 without verifying support. | Keep the defaults and parser contract. Update the documented supported range and the generated 19 settings. Acceptance by the parser alone is not compatibility. |
| [Core request](../../crates/pgconfig/src/request.rs) and [MCP](../../crates/pgconfig-server/src/mcp.rs) | `postgres_version` is required. MCP documents support through 18. | Update the range and prerelease explanation in the schema and [MCP contract](../mcp.md). Do not introduce a version default. |
| [Rules](../../crates/pgconfig/src/rules.rs) and [reasons](../../crates/pgconfig/src/reasons.rs) | One AIO rule applies to every version at least 18 and always emits `io_workers`. | Give 19 the dynamic-pool treatment. Keep 18 behavior separate. Every emitted replacement needs an accurate reason. |
| [Logging presets](../../crates/pgconfig/src/v1/mod.rs) | `pgbadger()` has no version argument and uses one preset for every release. | Gate `log_autoanalyze_min_duration=0` on 19 so earlier versions never receive an unknown setting. |
| [Golden cases](../../crates/golden/src/cases.rs) | The full matrix stops at 18, but REST and CLI input cases already include 19. Those fixtures contain `io_workers`. | Extend coverage and re-record the intended 19 changes. Review any affected permissive future-version cases as well. Never edit fixtures by hand. |
| [Integration workflow](../../.github/workflows/integration.yml) and [config check](../../scripts/check-conf-loads.sh) | The matrix ends at 18. One argument supplies both the numeric CLI version and the Docker image tag. | Separate the request version `19` from the beta image tag. Add a pinned beta-server check. |

The official Docker image manifest currently lists `postgres:19beta4`, not
a plain `postgres:19` alias. Passing `19beta4` to the existing CLI parser also
fails because it expects a number. A separate image tag, preferably pinned
by digest for CI, solves the two different version representations.
[Official image manifest, checked 2026-10-03](https://raw.githubusercontent.com/docker-library/official-images/master/library/postgres)

The version selector is a local web list. `/v1/version` reports the pgconfig
application version, not the supported PostgreSQL versions. No change to
that endpoint is needed. See the [REST routes](../../crates/pgconfig-server/src/v1/mod.rs).

The accepted [v1 compatibility ADR](../adr/0001-rust-engine-with-v1-frozen-by-goldens.md)
allows deliberate rule changes with reviewed golden updates. This support
work should preserve the legacy parser behavior and earlier-version numeric
recommendations. The eventual user-visible change needs a changeset under
[the release policy](../releases.md); this research document does not.

## Documentation findings before the merge

The [embedded catalog](../../pg-docs.yml) has 21 entries for PostgreSQL 18 and
no section for 19. The [generator parameter list](https://github.com/momoi-labs/pgconfig/blob/0fde6505012a610830c9c63854991e20f5f3bca9/generators/pg-docs/main.go)
contains 22 names across versions, including `checkpoint_segments`. It omits
the optional logging settings and `io_combine_limit`.

Observed gaps that affect a trustworthy 19 catalog:

- On 2026-10-03, fetching the generator's
  [19 shared_buffers URL](https://postgresqlco.nf/en/doc/param/shared_buffers/19/)
  returned HTTP 404 after redirecting to `/doc/en/param/shared_buffers/19/`.
  This confirms that the current source cannot supply even that required
  entry at that URL. It does not mean PostgreSQL removed `shared_buffers`.
- The generator treats every fetch error as an unsupported parameter, then
  saves the partial catalog. Generate the 19 entries from pinned official
  sources and check required coverage before replacing stored data. A failed
  fetch and a parameter intentionally absent in that release are different.
  See [processParam](https://github.com/momoi-labs/pgconfig/blob/0fde6505012a610830c9c63854991e20f5f3bca9/generators/pg-docs/main.go) and
  [Get](../../generators/pg-docs/docs.go).
- The scraper reads minimum and maximum values by row position. The stored
  18 metadata for `io_method` has `min_value: postmaster` and
  `max_value: "true"`; `file_copy_method` has `user` and `"false"`.
  These are context/restart fields, not bounds. String and enum entries need
  their own field handling. Do not carry these values into the 19 catalog.
- [rules.yml](../../rules.yml) still describes `effective_io_concurrency` as
  bitmap-scan-only and presents the fixed I/O worker percentage as general
  advice. Update those explanations for the applicable versions. Its
  `io_combine_limit` entry is documentation-only today; the engine does not
  emit it. The `max_worker_processes` text also needs to distinguish ordinary
  autovacuum workers from their parallel helpers.

Keep the existing manual text, project heuristics, and external recommendations
visibly distinct. A retained PostgreSQL default does not validate a project
formula. Review historical documentation corrections separately from importing
19 so the golden diff exposes each intended change.

## Evidence required before listing 19

Acceptance checks for the implementation:

- A fresh web form, a REST request without `pg_version`, and a CLI invocation
  without `--version` still choose 18. Core/MCP requests without a version
  still fail validation. Explicit `19` works throughout.
- Every generated 19 setting has valid name, type, units, bounds, and a
  version-specific explanation. `io_workers` is absent. Earlier versions
  never receive the new autoanalyze setting or dynamic-pool parameters.
- The five profiles and relevant storage/platform cases produce consistent
  values across JSON, conf, ALTER SYSTEM, and StackGres serialization. A valid
  StackGres document does not establish that an operator release supports 19.
- Re-record and review goldens through the documented command. Existing
  supported-version numeric output stays stable, and the 19 fixture changes
  reflect the reviewed policy. Preserve the explicit default-18 assertions.
- Load generated configurations in the pinned 19 beta server and inspect
  applied settings. Cover the pgBadger preset as well as the tuning settings.
  A Linux container does not verify Windows I/O limits. Resolve the DW limit
  before claiming that combination is supported.

At the research stage, source inventories were independently counted, and
all 22 core names plus all 13 logging names were matched against the manual.
The implementation checks are recorded below. New performance claims still
require measurements.


## Implementation validation

- `just test` passes, including the web tests, MCP and HTTP integration tests,
  tuning tests, extractor tests, and golden replay.
- `just lint` passes with formatting and Clippy warnings treated as errors.
- The 19 catalog has 429 manual entries. All 418 entries present in a standard
  x86-64 Linux build match `pg_settings` in `postgres:19beta4` for defaults,
  units, limits, enum values, context, and category. Only the three documented
  Debian differences remain: Kerberos path, socket directory, version suffix.
- Regenerating 18 from `REL_18_6` produces no catalog diff. `pg-docs.yml` is
  unchanged.
- Golden comparison: 6,074 existing cases are unchanged, 21 change only
  project notes, and seven change 19 or permissive future-version outputs.
  There are 434 additional cases. Every existing supported-version case
  through 18 keeps its values.
- Each of WEB, OLTP, DW, Mixed, and Desktop loads all generated settings in
  the pinned 19 beta 4 container, including JSON logging and the autoanalyze
  setting. The default 18 container check also passes.
- Browser checks cover the default 18 selection, selecting 19 beta, new pool
  rows, catalog-backed defaults, expanded manual text, official documentation
  links, and the pgBadger export. The 19 page does not emit `io_workers`.
- Windows bounds are checked against the tagged source and tuning tests.
  No Windows PostgreSQL server or pgBadger log parser was run. No performance
  benchmarks were run.
