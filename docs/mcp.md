# PGConfig MCP contract

PGConfig provides a public Model Context Protocol server for deterministic
PostgreSQL configuration recommendations. This document is the source of truth
for the initial public MCP contract.

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

Client configuration formats vary; select Streamable HTTP when a client asks
for a transport. The server identifies itself as `pgconfig` and reports the
same release version as the PGConfig API.

## Tool

The first release exposes exactly one tool:

```text
recommend_postgres_configuration
```

It is read-only, deterministic, and idempotent. A call accepts a complete or
partially defaulted Tuning Request and returns a structured Tuning Result. The
same result is also serialized as JSON text for clients that do not yet consume
MCP structured content.

The tool returns PostgreSQL parameter values and explanations. It does not
return a rendered `postgresql.conf`, `ALTER SYSTEM` statements, or StackGres
configuration, and it does not accept an output-format argument.

## Tuning Request

Input names use snake case. Required values must be supplied by the caller; the
service never detects resources from its own runtime because those resources
describe the PGConfig deployment, not the user's PostgreSQL server.

| Input | Required | Accepted values | Normalized value or default |
| --- | --- | --- | --- |
| `total_ram` | Yes | Positive integer followed by `B`, `KB`, `MB`, `GB`, or `TB`, case-insensitive | Unit is normalized; no default |
| `total_cpu` | Yes | Positive integer count of logical CPUs, including hyperthreads | Integer; no default |
| `postgres_version` | Yes | Dotted numeric string in supported series 9.1–9.6 or 10–18 | Complete supplied version is preserved; no default |
| `profile` | No | `WEB`, `OLTP`, `DW`, `MIXED`, or `DESKTOP`, case-insensitive | Uppercase; defaults to `WEB` |
| `disk_type` | No | `SSD`, `HDD`, or `SAN`, case-insensitive | Uppercase; defaults to `SSD` |
| `os` | No | `linux`, `windows`, `unix`, or `darwin`, case-insensitive | Lowercase; defaults to `linux` |
| `arch` | No | `386`, `i686`, `amd64`, `x86-64`, `arm`, or `arm64`, case-insensitive | `i686` becomes `386`; `x86-64` becomes `amd64`; defaults to `amd64` |
| `max_connections` | No | Positive integer with no arbitrary maximum | Integer; defaults to `100` |

`total_ram` deliberately rejects decimals, missing units, unknown units, zero,
and negative values. Express a fractional larger unit as an integer in a smaller
unit, for example `1536MB` instead of `1.5GB`.

`postgres_version` remains a string so values such as `9.6` and `17.10` are not
altered by numeric parsing. PGConfig derives the PostgreSQL Major Version using
the first two number groups before PostgreSQL 10 and the first group from
PostgreSQL 10 onward. Syntax and the supported series are validated, but the
tool does not attempt to verify whether a particular minor release was
published.

Invalid optional values are errors and never silently select their defaults.
Every omitted optional value that receives a default is reported as a Tuning
Assumption.

### Successful request

This request supplies every field, so its result has no default assumptions:

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
and the complete PostgreSQL Version `18.4`.

## Tuning Result

A successful call returns an object with these fields:

| Field | Meaning |
| --- | --- |
| `request` | Every normalized, supplied, or defaulted Tuning Request field |
| `assumptions` | Deterministic English descriptions of defaults applied by the server |
| `warnings` | Non-fatal concerns that should be reviewed; they do not invalidate the result |
| `recommendations` | Map keyed by PostgreSQL parameter name |
| `application_version` | PGConfig API release version that produced the result |

Each recommendation contains a PostgreSQL-formatted `value` and a concise,
deterministic English `reason`. Reasons describe the final value and mention a
cap or later adjustment when one changed the initial calculation.

The following is an abbreviated representative structured result. Actual
results contain every recommendation supported by the requested PostgreSQL
Major Version.

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
  "application_version": "3.6.0"
}
```

`listen_addresses` is intentionally absent because it controls connectivity and
has security implications rather than being a performance recommendation.

### Defaults and Tuning Assumptions

For example, a request containing only the required fields applies all five
optional defaults:

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
`linux`, architecture `amd64`, and `100` maximum connections. The `assumptions`
array contains one entry for each of those five defaults. Assumptions explain
facts the server supplied; warnings instead flag a non-fatal concern about a
usable request, such as an unusually high connection count.

## Errors

Missing or invalid inputs are returned as MCP tool execution errors. They do not
indicate a broken MCP transport. Clients should present the actionable English
message, correct the arguments, and call the tool again. Common error cases are:

- missing `total_ram`, `total_cpu`, or `postgres_version`;
- RAM without a unit, with a decimal, or with a non-positive value;
- non-positive or non-integer CPU and connection counts;
- malformed or unsupported PostgreSQL Versions;
- unknown profile, disk type, operating system, or architecture values.

For example, this request is invalid because two required facts are missing and
RAM has no unit:

```json
{
  "name": "recommend_postgres_configuration",
  "arguments": {
    "total_ram": "16"
  }
}
```

The tool result has `isError: true` and text content identifying the missing
fields and invalid RAM value so the caller can self-correct. Protocol or HTTP
failures should be treated separately from these execution errors.

## Operational contract

- Access is public and anonymous. The server stores no session or conversation
  state and performs no application-level caching.
- Native clients that omit the `Origin` header are accepted. Browser requests
  are accepted only when their Origin is one of the deployment's configured
  official PGConfig origins; unrelated browser origins are rejected.
- Each tool execution has a five-second timeout. A timeout is returned as an
  actionable tool execution error.
- The existing Fiber request body-size limit applies. There is no additional
  MCP-specific request-size setting.
- Existing deployment-edge protections apply. The initial release adds no
  custom application rate limiter.
- A successful execution emits a structured log with `tool`, `status`, duration
  in milliseconds, assumption count, warning count, and server version.
- A failed execution emits `status`, a stable error code, and missing-field names
  when applicable. Complete tool arguments and complete Tuning Requests are
  never logged.

## Deferred work

The first release intentionally keeps one small, stateless, read-only tool.
Deferred items are recorded with the condition that would justify revisiting
them:

| Deferred item | Why it is deferred | Revisit when |
| --- | --- | --- |
| Application caching | Calculation cost and invalidation complexity are not yet justified | Measured latency or compute cost becomes operationally significant |
| Custom rate limiting | Existing Cloudflare protections are the initial baseline | Traffic or abuse exceeds edge protection |
| Prometheus metrics | Structured logs provide initial observability | PGConfig has a metrics collector and monitoring infrastructure |
| Authentication | The tool is public, stateless, and read-only | It gains private data, persisted state, user-specific behavior, or mutations |
| Additional tools | One intent has one clear tool | A distinct user intent cannot be expressed cleanly by `recommend_postgres_configuration` |
| Detailed calculation traces | Concise reasons provide useful provenance without a large schema | Consumers need machine-readable, step-by-step provenance |
| pgBadger and `log_format` | Log analysis is a separate observability-driven capability | That capability has its own requirements and design |
| Decimal RAM | Equivalent integer values can be supplied in smaller units | Real clients cannot reliably express those equivalent values |
| `listen_addresses` review | Connectivity guidance is security-sensitive and separate from tuning | Operational experience supports secure, explicit connectivity guidance |

Submission to the official MCP Registry follows endpoint and documentation
stabilization. Registry publication does not block the initial deployment.
