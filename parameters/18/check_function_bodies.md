---
name: "check_function_bodies"
version: "18"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Check routine bodies during CREATE FUNCTION and CREATE PROCEDURE."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/18/runtime-config-client.html#GUC-CHECK-FUNCTION-BODIES"
---

This parameter is normally on. When set to `off`, it disables validation of the routine body string during [CREATE FUNCTION](https://www.postgresql.org/docs/18/sql-createfunction.html) and [CREATE PROCEDURE](https://www.postgresql.org/docs/18/sql-createprocedure.html). Disabling validation avoids side effects of the validation process, in particular preventing false positives due to problems such as forward references. Set this parameter to `off` before loading functions on behalf of other users; pg_dump does so automatically.
