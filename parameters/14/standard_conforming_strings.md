---
name: "standard_conforming_strings"
version: "14"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Causes '...' strings to treat backslashes literally."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/14/runtime-config-compatible.html#GUC-STANDARD-CONFORMING-STRINGS"
---

This controls whether ordinary string literals (`'...'`) treat backslashes literally, as specified in the SQL standard. Beginning in PostgreSQL 9.1, the default is `on` (prior releases defaulted to `off`). Applications can check this parameter to determine how string literals will be processed. The presence of this parameter can also be taken as an indication that the escape string syntax (`E'...'`) is supported. Escape string syntax ([String Constants with C-Style Escapes](https://www.postgresql.org/docs/14/sql-syntax-lexical.html#SQL-SYNTAX-STRINGS-ESCAPE)) should be used if an application desires backslashes to be treated as escape characters.
