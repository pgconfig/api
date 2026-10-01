# PGConfig MCP contract

PGConfig provides a public Model Context Protocol server for deterministic
PostgreSQL configuration recommendations. This document is the source of truth
for the public MCP contract.

The endpoint is:

```text
https://api.pgconfig.org/mcp
```

Configure that URL as a remote **Streamable HTTP** server in any compatible MCP
client. The connection is anonymous, so do not configure an authorization
header, API key, or OAuth flow. For example, clients that use an MCP server map
can represent the connection as:

```json
{
  "mcpServers": {
    "pgconfig": {
      "url": "https://api.pgconfig.org/mcp"
    }
  }
}
```

Client configuration formats vary. Select Streamable HTTP when a client asks
for a transport. The server identifies itself as `pgconfig` and reports the
same release version as the PGConfig API.

## Tool

The server exposes exactly one tool:

```text
recommend_postgres_configuration
```

It is read-only, deterministic, and idempotent. A call accepts a complete or
partially defaulted Tuning Request and returns a structured Tuning Result. The
same result is also serialized as JSON text for clients that do not consume MCP
structured content.

The tool returns PostgreSQL parameter values and explanations. It does not
return a rendered `postgresql.conf`, `ALTER SYSTEM` statements, or StackGres
configuration, and it does not accept an output-format argument. REST v1 and
`pgconfigctl` produce those.

## Tuning Request

Argument names use snake case. Required values must be supplied by the caller.
The service never detects resources from its own runtime, because those
resources describe the PGConfig deployment and not the user's PostgreSQL
server.

| Argument | Required | Accepted values | Normalized value or default |
| --- | --- | --- | --- |
| `total_ram` | Yes | String: a positive integer followed by `B`, `KB`, `MB`, `GB`, or `TB`, in any case | Uppercase unit, largest unit that divides the amount. No default |
| `total_cpu` | Yes | Positive integer count of logical CPUs, including hyperthreads | Integer. No default |
| `postgres_version` | Yes | String of dotted numbers in a supported series: 9.1 to 9.6, or 10 to 18 | The supplied version is kept as it is. No default |
| `profile` | No | `WEB`, `OLTP`, `DW`, `MIXED`, or `DESKTOP`, in any case | Uppercase. Defaults to `WEB` |
| `disk_type` | No | `SSD`, `HDD`, or `SAN`, in any case | Uppercase. Defaults to `SSD` |
| `os` | No | `linux`, `windows`, `unix`, or `darwin`, in any case | Lowercase. Defaults to `linux` |
| `arch` | No | `386`, `i686`, `amd64`, `x86-64`, `arm`, or `arm64`, in any case | `i686` becomes `386` and `x86-64` becomes `amd64`. Defaults to `amd64` |
| `max_connections` | No | Positive integer with no upper limit | Integer. Defaults to `100` |

`total_ram` rejects decimals, missing units, unknown units, zero, and negative
values. Write a fractional larger unit as an integer in a smaller one, for
example `1536MB` instead of `1.5GB`.

`postgres_version` is a string so that `9.6` and `17.10` are not altered by
numeric parsing. A JSON number is rejected. PGConfig derives the PostgreSQL
Major Version from the first two number groups before PostgreSQL 10 and from
the first group from PostgreSQL 10 onward. It checks the syntax and the
supported series. It does not check whether a particular minor release was
published.

An invalid optional value is an error and never selects the default. Every
omitted optional value that receives a default is reported as a Tuning
Assumption.

The tool's input schema describes the accepted names in each description
instead of an `enum`, so a client that validates arguments against the schema
does not reject the lowercase spellings.

### Successful request

This request supplies every argument, so its result has no assumptions:

```json
{
  "name": "recommend_postgres_configuration",
  "arguments": {
    "total_ram": "16GB",
    "total_cpu": 8,
    "postgres_version": "18.4",
    "profile": "web",
    "disk_type": "ssd",
    "os": "Linux",
    "arch": "x86-64",
    "max_connections": 100
  }
}
```

The normalized request in the result contains `WEB`, `SSD`, `linux`, `amd64`,
and the PostgreSQL Version `18.4`.

## Tuning Result

A successful call returns an object with these fields:

| Field | Meaning |
| --- | --- |
| `request` | Every Tuning Request field, normalized, supplied or defaulted |
| `assumptions` | One entry per default the server applied: `field`, `value`, and an English `message` |
| `warnings` | Concerns to review that do not invalidate the result: a stable `code` and an English `message` |
| `recommendations` | Map keyed by PostgreSQL parameter name |
| `application_version` | The PGConfig release and commit that produced the result |

Each recommendation contains a PostgreSQL-formatted `value` and a short,
deterministic English `reason`. The reason describes the final value and names
any limit or later adjustment that changed the initial calculation. A
parameter that the requested PostgreSQL Major Version does not have is absent.

The following is an abbreviated structured result. A real result contains
every recommendation the requested PostgreSQL Major Version supports.

```json
{
  "request": {
    "os": "linux",
    "arch": "amd64",
    "total_ram": "16GB",
    "profile": "WEB",
    "disk_type": "SSD",
    "max_connections": 100,
    "total_cpu": 8,
    "postgres_version": "18.4"
  },
  "assumptions": [],
  "warnings": [],
  "recommendations": {
    "shared_buffers": {
      "value": "4GB",
      "reason": "Set to 4GB from the memory share for the WEB profile."
    },
    "max_connections": {
      "value": "100",
      "reason": "Set to 100 to match the requested connection limit."
    }
  },
  "application_version": "3.6.1 (507fdd3)"
}
```

`listen_addresses` is absent on purpose. It controls who may connect, which is
a security decision and not a performance recommendation.

### Defaults and Tuning Assumptions

A request with only the required arguments applies all five defaults:

```json
{
  "name": "recommend_postgres_configuration",
  "arguments": {
    "total_ram": "8GB",
    "total_cpu": 4,
    "postgres_version": "17.10"
  }
}
```

Its normalized request uses profile `WEB`, disk type `SSD`, operating system
`linux`, architecture `amd64`, and `100` maximum connections. The
`assumptions` array has one entry for each of the five:

```json
{
  "field": "profile",
  "value": "WEB",
  "message": "profile was not supplied. Assumed WEB."
}
```

### Warnings

An assumption explains a fact the server supplied. A warning flags a concern
about a usable request. There is one warning today:

| Code | When |
| --- | --- |
| `high_max_connections` | `max_connections` is above 1000 |

## Errors

Missing or invalid arguments come back as an MCP tool execution error: the
tool result has `isError: true` and one text content block. They do not
indicate a broken MCP transport. One error lists every problem with the call,
in argument order, so the caller can fix them all and call again. The error
cases are:

- a missing `total_ram`, `total_cpu`, or `postgres_version`;
- RAM without a unit, with a decimal, or with a non-positive value;
- a non-positive or non-integer CPU or connection count;
- a malformed or unsupported PostgreSQL Version;
- an unknown profile, disk type, operating system, or architecture;
- an argument of the wrong JSON type, such as a number for `postgres_version`;
- an argument the tool does not have.

For example, this request is missing two required facts and its RAM has no
unit:

```json
{
  "name": "recommend_postgres_configuration",
  "arguments": {
    "total_ram": "16"
  }
}
```

Its error text is:

```text
Invalid tuning request. total_ram: "16" has no unit. Use a positive integer followed by B, KB, MB, GB, or TB, such as 16GB or 1536MB. total_cpu is required: the number of logical CPUs, such as 8. postgres_version is required: the PostgreSQL version, such as 18.4.
```

A call to a tool name the server does not have is a protocol error
(`-32602`). Treat protocol and HTTP failures separately from tool execution
errors.

## Operational contract

- Access is public and anonymous. The server keeps no session and no
  conversation state, and it does no application-level caching. Every call is
  one POST answered with one JSON response.
- A native client sends no `Origin` header and is accepted. A browser request
  is accepted only when its `Origin` is one of the deployment's configured
  origins, and every other origin gets HTTP 403, preflight included. The
  default origins are `https://pgconfig.org` and `https://www.pgconfig.org`.
  A deployment changes them with `--mcp-allowed-origins` or
  `PGCONFIG_MCP_ALLOWED_ORIGINS`, separated by commas.
- The `Host` header is not checked. That check protects a local server from
  DNS rebinding, and this endpoint is public and read-only.
- Each tool execution has a five-second timeout. A timeout comes back as a
  tool execution error that asks the caller to try again.
- A request body above 4 MiB gets HTTP 413.
- The protections at the deployment edge apply. The server has no rate limiter
  of its own.
- A successful execution logs `tool`, `status`, `duration_ms`, the assumption
  count, the warning count, and `server_version`.
- A failed execution logs `status`, a stable `error_code` (`invalid_request` or
  `timeout`), and the names of the missing fields. The arguments and the
  Tuning Request are never logged.

## Conformance

CI runs the official MCP conformance suite, pinned to one version, against the
server on every pull request: the `server-initialize`, `ping`, and
`tools-list` scenarios. The remaining server scenarios exercise features this
server does not have, such as prompts, resources, and sampling. The
`dns-rebinding-protection` scenario requires a server that only answers on
localhost, which a public endpoint cannot be.

## Deferred work

The server keeps one small, stateless, read-only tool on purpose. Each deferred
item is recorded with the condition that would justify revisiting it:

| Deferred item | Why it is deferred | Revisit when |
| --- | --- | --- |
| Application caching | Calculation cost does not justify the invalidation work | Measured latency or compute cost becomes significant in operation |
| Custom rate limiting | The deployment edge is the baseline | Traffic or abuse exceeds what the edge handles |
| Prometheus metrics | Structured logs are the first observability mechanism | PGConfig has a metrics collector and monitoring infrastructure |
| Authentication | The tool is public, stateless, and read-only | It gains private data, persisted state, user-specific behavior, or mutations |
| Additional tools | One intent has one clear tool | A distinct user intent cannot be expressed by `recommend_postgres_configuration` |
| Detailed calculation traces | Short reasons give provenance without a large schema | Consumers need machine-readable, step-by-step provenance |
| pgBadger and `log_format` | Log analysis is a separate capability | That capability has its own requirements and design |
| Decimal RAM | An integer in a smaller unit expresses the same amount | Real clients cannot express those values reliably |
| `listen_addresses` review | Connectivity guidance is security-sensitive and separate from tuning | Operational experience supports explicit, secure connectivity guidance |
| REST v2 | Its shape was never specified | The rich Tuning Result needs an HTTP JSON interface outside MCP |

Submission to the official MCP Registry follows the stabilization of the
endpoint and of this document. It does not block the first deployment.
