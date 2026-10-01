# Profiles

A profile describes the workload the server runs. It decides how much of the
memory PostgreSQL may use and how the write and parallel settings are sized.
Pass it as `environment_name`.

| Profile | Workload | Typical use |
| --- | --- | --- |
| `WEB` | General web applications | A portal or a corporate application |
| `OLTP` | Many short transactions | An ERP, or a system with many simultaneous writes |
| `DW` | Few large analytical queries | Data warehouse and business intelligence |
| `MIXED` | The database shares the server with the application | Small applications on one machine |
| `DESKTOP` | A development machine | Development, support, or a demo |

## What changes with the profile

| Setting | `WEB` | `OLTP` | `DW` | `MIXED` | `DESKTOP` |
| --- | --- | --- | --- | --- | --- |
| Share of RAM PostgreSQL may use | All | All | All | Half | One fifth |
| Share of that memory for all the `work_mem` together | 25% | 35% | 50% | 20% | 10% |
| `min_wal_size` and `max_wal_size` | 1GB, 4GB | 2GB, 8GB | 4GB, 16GB | 2GB, 6GB | 512MB, 2GB |
| `wal_buffers` | Automatic | 32MB above 32GB of RAM, automatic below | 64MB | Automatic | Automatic |
| `max_parallel_workers_per_gather` | 2 | 2 | Half the CPUs, at least 2 | 2 | 2 |
| `io_max_combine_limit` (PostgreSQL 18) | 16 | 16 | 128 | 16 | 16 |
| `io_max_concurrency` (PostgreSQL 18) | 64 | 128 | 256 | 64 | 64 |

`shared_buffers` is a quarter of the profile's memory. `DESKTOP` is the
exception: it uses one sixteenth of the total RAM. Automatic means
`wal_buffers = -1`, which lets PostgreSQL choose the size.

The web app shows the five profiles side by side for your server. See the
[profile comparison](/).
