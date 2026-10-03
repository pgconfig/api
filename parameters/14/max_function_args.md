---
name: "max_function_args"
version: "14"
type: "integer"
category: "Preset Options"
short_desc: "Shows the maximum number of function arguments."
context: "internal"
default: "100"
min: "100"
max: "100"
url: "https://www.postgresql.org/docs/14/runtime-config-preset.html#GUC-MAX-FUNCTION-ARGS"
---

Reports the maximum number of function arguments. It is determined by the value of `FUNC_MAX_ARGS` when building the server. The default value is 100 arguments.
