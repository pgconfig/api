---
name: "debug_write_read_parse_plan_trees"
version: "19"
type: "boolean"
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-DEBUG-WRITE-READ-PARSE-PLAN-TREES"
---

Enabling this forces all parse and plan trees to be passed through `outfuncs.c`/`readfuncs.c`, to facilitate catching errors and omissions in those modules. The default is off.

This parameter is only available when `DEBUG_NODE_TESTS_ENABLED` was defined at compile time (which happens automatically when using the configure option `--enable-cassert`).
