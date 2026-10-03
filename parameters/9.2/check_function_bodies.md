---
name: "check_function_bodies"
version: "9.2"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Check function bodies during CREATE FUNCTION."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.2/runtime-config-client.html#GUC-CHECK-FUNCTION-BODIES"
---

This parameter is normally on. When set to `off`, it disables validation of the function body string during [CREATE FUNCTION](https://www.postgresql.org/docs/9.2/sql-createfunction.html). Disabling validation avoids side effects of the validation process and avoids false positives due to problems such as forward references. Set this parameter to `off` before loading functions on behalf of other users; pg_dump does so automatically.
