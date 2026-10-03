---
name: "standard_conforming_strings"
version: "19"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Nonstandard strings are no longer supported; this can only be true."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-compatible.html#GUC-STANDARD-CONFORMING-STRINGS"
---

Beginning in PostgreSQL 19, this parameter is always `on`. String literals are always parsed as specified in the SQL standard (that is, backslashes are ordinary characters within a string literal). This parameter continues to exist because applications may consult it; but it cannot be set to `off`. Escape string syntax ([String Constants with C-Style Escapes](https://www.postgresql.org/docs/19/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-ESCAPE)) should be used if an application desires backslashes to be treated as escape characters.
