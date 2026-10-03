---
name: "sql_inheritance"
version: "9.5"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Causes subtables to be included by default in various commands."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.5/runtime-config-compatible.html#GUC-SQL-INHERITANCE"
---

This setting controls whether undecorated table references are considered to include inheritance child tables. The default is `on`, which means child tables are included (thus, a `*` suffix is assumed by default). If turned `off`, child tables are not included (thus, an `ONLY` prefix is assumed). The SQL standard requires child tables to be included, so the `off` setting is not spec-compliant, but it is provided for compatibility with PostgreSQL releases prior to 7.1. See [Inheritance](https://www.postgresql.org/docs/9.5/ddl-inherit.html) for more information.

Turning `sql_inheritance` off is deprecated, because that behavior has been found to be error-prone as well as contrary to SQL standard. Discussions of inheritance behavior elsewhere in this manual generally assume that it is `on`.
