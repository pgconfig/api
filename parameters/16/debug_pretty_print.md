---
name: "debug_pretty_print"
version: "16"
type: "boolean"
category: "Reporting and Logging / What to Log"
short_desc: "Indents parse and plan tree displays."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/16/runtime-config-logging.html#GUC-DEBUG-PRETTY-PRINT"
---

When set, `debug_pretty_print` indents the messages produced by `debug_print_parse`, `debug_print_rewritten`, or `debug_print_plan`. This results in more readable but much longer output than the "compact" format used when it is off. It is on by default.
