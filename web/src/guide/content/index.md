# PGConfig documentation

PGConfig recommends PostgreSQL configuration values from the facts about a
server: memory, CPUs, PostgreSQL version, workload, and storage.

There are four ways to use it. They share one set of rules, so they agree on
the values.

| Interface | Use it when | Start here |
| --- | --- | --- |
| Web app | You want to compare the profiles and copy a configuration | [Profile comparison](/) |
| REST v1 | A script or an installer needs a configuration | [Get a configuration](/guide/api) |
| `pgconfigctl` | You are on the server and want to tune it from the shell | [Releases](https://github.com/momoi-labs/pgconfig/releases) |
| MCP | An AI agent should get values together with the reason for each | [MCP](/guide/mcp) |

## Quick start

Ask the API for a `postgresql.conf` fragment:

```bash
curl 'https://api.pgconfig.org/v1/tuning/get-config?total_ram=16GB&cpus=8&pg_version=17&format=conf'
```

Or run the CLI on the server. Without flags it reads the machine's memory and
CPUs:

```bash
pgconfigctl tune
```

## What stays stable

REST v1 does not change. Installers call it unattended, so its routes,
parameters, defaults, and output are frozen, quirks included. New behavior
arrives in new interfaces, such as MCP.

The OpenAPI document of REST v1 is at [/docs](/docs/).
