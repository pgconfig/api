---
name: "default_with_oids"
version: "10"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Create new tables with OIDs by default."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/10/runtime-config-compatible.html#GUC-DEFAULT-WITH-OIDS"
---

This controls whether `CREATE TABLE` and `CREATE TABLE AS` include an OID column in newly-created tables, if neither `WITH OIDS` nor `WITHOUT OIDS` is specified. It also determines whether OIDs will be included in tables created by `SELECT INTO`. The parameter is `off` by default; in PostgreSQL 8.0 and earlier, it was `on` by default.

The use of OIDs in user tables is considered deprecated, so most installations should leave this variable disabled. Applications that require OIDs for a particular table should specify `WITH OIDS` when creating the table. This variable can be enabled for compatibility with old applications that do not follow this behavior.
