---
name: "quote_all_identifiers"
version: "9.6"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "When generating SQL fragments, quote all identifiers."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/9.6/runtime-config-compatible.html#GUC-QUOTE-ALL-IDENTIFIERS"
---

When the database generates SQL, force all identifiers to be quoted, even if they are not (currently) keywords. This will affect the output of `EXPLAIN` as well as the results of functions like `pg_get_viewdef`. See also the `--quote-all-identifiers` option of [pg_dump](https://www.postgresql.org/docs/9.6/app-pgdump.html) and [pg_dumpall](https://www.postgresql.org/docs/9.6/app-pg-dumpall.html).
