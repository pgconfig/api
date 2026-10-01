# PGConfig

PGConfig recommends PostgreSQL configuration values from the facts about a
server: memory, CPUs, PostgreSQL version, workload, and storage. It runs
[pgconfig.org](https://pgconfig.org).

This repository holds all of it, in Rust:

| Part | What it is |
| --- | --- |
| `crates/pgconfig` | The tuning engine. No I/O |
| `crates/pgconfig-server` | One binary that serves REST v1, the web app, the guide, and MCP |
| `crates/pgconfigctl` | The command-line tool |
| `web/` | The web app, built with React and embedded in the server |

## Use it

### Command line

Download `pgconfigctl` from the
[releases](https://github.com/momoi-labs/pgconfig/releases), as a binary, a
deb, or an rpm. Without flags it reads the memory and CPUs of the machine it
runs on:

```sh
pgconfigctl tune
pgconfigctl tune --ram 16GB --cpus 8 --version 17 --profile OLTP --format sql
```

`pgconfigctl tune --help` lists the flags. The formats are `conf`,
`alter_system`, `stackgres`, and `json`.

### REST v1

```sh
curl 'https://api.pgconfig.org/v1/tuning/get-config?total_ram=16GB&cpus=8&pg_version=17&format=conf'
```

The OpenAPI document and the Swagger UI are at `/docs`. REST v1 is stable:
installers call it unattended, so its behavior does not change.

### MCP

AI agents can ask for tuning recommendations over the Model Context Protocol at
`https://api.pgconfig.org/mcp`. Each recommendation comes with the reason for
its value. The contract is in [docs/mcp.md](docs/mcp.md).

### Run the server

```sh
docker run --rm -p 3000:3000 ghcr.io/momoi-labs/pgconfig
```

Then open <http://localhost:3000>. The same port serves the web app, the guide
under `/guide`, REST v1 under `/v1`, and MCP at `/mcp`. `PORT` changes the port.

## Docker images

- `ghcr.io/momoi-labs/pgconfig` runs `pgconfig-server`.
- `ghcr.io/momoi-labs/pgconfigctl` runs `pgconfigctl`.

The images `pgconfig/api` and `pgconfig/pgconfigctl`, on Docker Hub and on
`ghcr.io/pgconfig`, stopped at 3.6.1. `ghcr.io/momoi-labs/pgconfig` replaces
`pgconfig/api`: its binary is `pgconfig-server`, and it needs no `rules.yml` or
`pg-docs.yml` next to it.

## CPU Core Counting

The `cpus` argument is the total number of **logical CPU cores**, which
includes hyperthreading. This is the standard output from:
- Linux/Unix: `nproc` command
- Windows: Total processor count in Task Manager

**Example**: A system with 8 physical cores and hyperthreading enabled has 16 logical cores. Use `cpus=16`.

**Why logical cores?** Modern PostgreSQL (2017-2025) benefits from hyperthreading with [up to 15% performance improvement](https://www.cybertec-postgresql.com/en/experimenting-scaling-full-parallelism-postgresql/). The tuning formulas for `max_worker_processes`, `max_parallel_workers`, and `io_workers` are designed to work with logical core counts.

## Rules Engine

The configuration starts from the memory share of the profile and is then
adjusted by the environment.

| Category       | Condition                                   | Action/Adjustment                                                                                             |
| :------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------ |
| **Architecture**   | 32-bit (`386`, `i686`)                      | Cap `shared_buffers`, `work_mem`, `maintenance_work_mem` at 4GB.                                                |
| **OS**         | Windows & PG Version < 18                   | Cap `work_mem` and `maintenance_work_mem` at 2097151kB.                                                       |
| **Profile**    | `Desktop`                                   | Set `shared_buffers` to Total RAM / 16.                                                                       |
|                | `DW`                                        | Set `wal_buffers` to 64MB and raise the parallel and asynchronous I/O settings.                               |
|                | `OLTP` with more than 8GB of `shared_buffers` | Set `wal_buffers` to 32MB.                                                                                  |
| **Storage**    | Disk Type is `SSD`                          | Set `effective_io_concurrency` to `200` and `random_page_cost` to `1.1` (`1.8` for `DW`).                      |
|                | Disk Type is `SAN`                          | Set `effective_io_concurrency` to `300` and `random_page_cost` to `1.1` (`1.8` for `DW`).                      |
|                | Disk Type is `HDD`                          | Set `effective_io_concurrency` to `2`.                                                                        |
| **PG Version** | <= 9.6                                      | Cap `shared_buffers` at 512MB.                                                                                |
|                | < 9.6                                       | Remove `max_parallel_workers_per_gather`. Cap `shared_buffers` at 8GB.                                         |
|                | < 9.5                                       | Remove `min_wal_size` and `max_wal_size`. Keep `checkpoint_segments`.                                         |
|                | < 9.4                                       | Remove the worker settings.                                                                                   |
|                | < 10                                        | Remove `max_parallel_workers`.                                                                                |
|                | < 13                                        | Remove `maintenance_io_concurrency`.                                                                          |
|                | < 18                                        | Remove the asynchronous I/O settings (`io_method`, `io_workers`, and the others).                             |

## Development

The toolchain is pinned in `mise.toml`: Rust and Node, which only builds the
web app. With [just](https://github.com/casey/just):

```sh
just web     # build the web app, which the server embeds
just test    # web tests, then cargo test
just lint    # cargo fmt --check and cargo clippy
just run     # serve everything on http://localhost:3000
```

`cargo test` replays the golden files in `tests/golden` against the server and
the CLI. They were recorded from the Go implementation this one replaced, and
they pin REST v1 and the CLI output. See
[tests/golden/README.md](tests/golden/README.md).

More for contributors: [AGENTS.md](AGENTS.md), the decisions in
[docs/adr](docs/adr), and [docs/releases.md](docs/releases.md).

## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpgconfig%2Fapi.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpgconfig%2Fapi?ref=badge_large)
