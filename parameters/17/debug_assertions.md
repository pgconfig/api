---
name: "debug_assertions"
version: "17"
type: "boolean"
category: "Preset Options"
short_desc: "Shows whether the running server has assertion checks enabled."
context: "internal"
default: "off"
url: "https://www.postgresql.org/docs/17/runtime-config-preset.html#GUC-DEBUG-ASSERTIONS"
---

Reports whether PostgreSQL has been built with assertions enabled. That is the case if the macro `USE_ASSERT_CHECKING` is defined when PostgreSQL is built (accomplished e.g., by the `configure` option `--enable-cassert`). By default PostgreSQL is built without assertions.
