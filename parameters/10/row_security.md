---
name: "row_security"
version: "10"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Enable row security."
extra_desc: "When enabled, row security will be applied to all users."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-ROW-SECURITY"
---

This variable controls whether to raise an error in lieu of applying a row security policy. When set to `on`, policies apply normally. When set to `off`, queries fail which would otherwise apply at least one policy. The default is `on`. Change to `off` where limited row visibility could cause incorrect results; for example, pg_dump makes that change by default. This variable has no effect on roles which bypass every row security policy, to wit, superusers and roles with the `BYPASSRLS` attribute.

For more information on row security policies, see [CREATE POLICY](https://www.postgresql.org/docs/10/sql-createpolicy.html).
