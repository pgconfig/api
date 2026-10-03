---
name: "debug_raw_expression_coverage_test"
version: "19"
type: "boolean"
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-DEBUG-RAW-EXPRESSION-COVERAGE-TEST"
---

Enabling this forces all raw parse trees for DML statements to be scanned by `raw_expression_tree_walker()`, to facilitate catching errors and omissions in that function. The default is off.

This parameter is only available when `DEBUG_NODE_TESTS_ENABLED` was defined at compile time (which happens automatically when using the configure option `--enable-cassert`).
