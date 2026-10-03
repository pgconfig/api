# Example and rules

## A full example

This is the call the web app makes for its export panel: an OLTP server with
16GB of RAM and 8 CPUs on SSD, as `ALTER SYSTEM` statements, with the logging
settings for pgBadger.

```bash
curl 'https://api.pgconfig.org/v1/tuning/get-config?pg_version=17&total_ram=16GB&cpus=8&max_connections=100&environment_name=OLTP&drive_type=SSD&os_type=linux&arch=amd64&format=alter_system&include_pgbadger=true&log_format=jsonlog'
```

## How the values are calculated

Every interface computes its values in one place:
[`crates/pgconfig/src/rules.rs`](https://github.com/momoi-labs/pgconfig/blob/main/crates/pgconfig/src/rules.rs).

The calculation starts from the memory the profile may use (see
[Profiles](/guide/environment)):

| Setting | Starting value |
| --- | --- |
| `shared_buffers` | 25% of the profile's memory |
| `effective_cache_size` | 75% of the profile's memory |
| `work_mem` | The profile's `work_mem` share, divided by `max_connections` |
| `maintenance_work_mem` | 5% of the profile's memory |
| `max_worker_processes`, `max_parallel_workers` | The number of CPUs, and at least 8 |

Then the environment adjusts those values, in this order:

| Step | Condition | Adjustment |
| --- | --- | --- |
| Architecture | 32-bit (`386`, `i686`) | Cap `shared_buffers`, `work_mem`, and `maintenance_work_mem` at 4GB |
| PostgreSQL version | 9.6 or older | Cap `shared_buffers` at 512MB |
| Operating system | Windows before PostgreSQL 18 | Cap `work_mem` and `maintenance_work_mem` at 2097151kB |
| Profile | `DESKTOP` | Set `shared_buffers` to the total RAM divided by 16. This replaces the caps above |
| PostgreSQL version | Older than 9.6 | Cap `shared_buffers` at 8GB |

The storage settings depend on `drive_type`:

| `drive_type` | `effective_io_concurrency` | `random_page_cost` |
| --- | --- | --- |
| `HDD` | 2 | 4.0 |
| `SSD` | 200 | 1.1, or 1.8 for `DW` |
| `SAN` | 300 | 1.1, or 1.8 for `DW` |

From PostgreSQL 13, `maintenance_io_concurrency` gets the value of
`effective_io_concurrency`.

On PostgreSQL 18, `io_workers` is a share of the CPUs,
rounded up: 10% for `DESKTOP`, 20% for `WEB`, 25% for `MIXED`, 30% for `OLTP`,
and 40% for `DW`, plus 10% on HDD. The result is at least 2, and at most the
number of CPUs or 32, whichever is lower.

PostgreSQL 19 replaces `io_workers` with a dynamic pool. pgconfig emits the
PostgreSQL defaults, `io_min_workers=2` and `io_max_workers=8`, for every
profile. It leaves the pool timers at their defaults. These values preserve
the pool's ability to grow and shrink; they are not performance measurements.
The 19 pgBadger preset also sets `log_autoanalyze_min_duration=0`, since
vacuum and analyze logging now have separate controls.

For PostgreSQL 19 on Windows, `io_max_combine_limit` is capped at 16 blocks,
or 128kB with the usual 8kB block size. Raising this ceiling on Linux does
not change the active `io_combine_limit`.

Last, the settings the PostgreSQL version does not have are left out:

| PostgreSQL version | Left out |
| --- | --- |
| 19 and later | `io_workers` |
| Older than 19 | `io_min_workers`, `io_max_workers`, `log_autoanalyze_min_duration` |
| Older than 18 | `io_method`, `io_workers`, `io_max_combine_limit`, `io_max_concurrency`, `file_copy_method` |
| Older than 13 | `maintenance_io_concurrency` |
| Older than 10 | `max_parallel_workers` |
| Older than 9.6 | `max_parallel_workers_per_gather` |
| Older than 9.5 | `min_wal_size`, `max_wal_size`. `checkpoint_segments` appears instead |
| Older than 9.4 | `max_worker_processes` |

## Want the reason for each value?

REST v1 returns values only. The [MCP](/guide/mcp) tool returns each value
with the reason for it, including any cap that changed it.
