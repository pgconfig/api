---
name: "operator_precedence_warning"
version: "9.5"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Emit a warning for constructs that changed meaning since PostgreSQL 9.4."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/9.5/runtime-config-compatible.html#GUC-OPERATOR-PRECEDENCE-WARNING"
---

When on, the parser will emit a warning for any construct that might have changed meanings since PostgreSQL 9.4 as a result of changes in operator precedence. This is useful for auditing applications to see if precedence changes have broken anything; but it is not meant to be kept turned on in production, since it will warn about some perfectly valid, standard-compliant SQL code. The default is `off`.

See [Operator Precedence](https://www.postgresql.org/docs/9.5/sql-syntax-lexical.html#SQL-PRECEDENCE) for more information.
