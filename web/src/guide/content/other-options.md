# Other endpoints

Every answer has the envelope that [get-config](/guide/api) shows: `data`,
`jsonapi`, `links`, and `meta`. The examples on this page show `data` only.

## Every profile at once

`GET /v1/tuning/get-config-all-environments` answers the settings of the five
profiles for one server. The web app draws its comparison from this route.

It takes the parameters of [get-config](/guide/api), except
`environment_name`, `format`, and `include_pgbadger`, which it ignores. The
answer is always JSON.

```bash
curl 'https://api.pgconfig.org/v1/tuning/get-config-all-environments?total_ram=16GB&cpus=8'
```

```json
{
  "data": [
    { "environment": "WEB", "configuration": [] },
    { "environment": "OLTP", "configuration": [] },
    { "environment": "DW", "configuration": [] },
    { "environment": "MIXED", "configuration": [] },
    { "environment": "DESKTOP", "configuration": [] }
  ]
}
```

Each `configuration` holds the same categories that `get-config` returns in
`data`. They are left empty here to keep the example short.

## List the profiles

`GET /v1/tuning/list-environments` answers the profile names.

```json
{
  "data": ["WEB", "OLTP", "DW", "MIXED", "DESKTOP"]
}
```

## The version of the API

`GET /v1/version` answers the release that is running and the commit it was
built from.

```json
{
  "data": {
    "build": "1a2b3c4",
    "pretty": "4.0.0 (1a2b3c4)",
    "version": "4.0.0"
  }
}
```

## The OpenAPI document

The Swagger UI is at [/docs](/docs/), and the OpenAPI 3.1 document at
[/docs/openapi.json](/docs/openapi.json).

## Parameter documentation

`GET /parameters/<version>/<name>.md` answers the PostgreSQL manual's entry for
one parameter of one major version, such as `18` or `9.6`. It is not part of
REST v1 and has no envelope: the answer is a Markdown file with the settings as
YAML front matter, then the manual's text. An unknown version or parameter
gets HTTP 404.

```bash
curl 'https://api.pgconfig.org/parameters/18/work_mem.md'
```

```markdown
---
name: "work_mem"
version: "18"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used for query workspaces."
context: "user"
unit: "kB"
default: "4096"
min: "64"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-WORK-MEM"
---

Sets the base maximum amount of memory to be used by a query operation...
```
