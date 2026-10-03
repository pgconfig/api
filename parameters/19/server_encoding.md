---
name: "server_encoding"
version: "19"
type: "string"
category: "Preset Options"
short_desc: "Shows the server (database) character set encoding."
context: "internal"
default: "SQL_ASCII"
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-SERVER-ENCODING"
---

Reports the database encoding (character set). It is determined when the database is created. Ordinarily, clients need only be concerned with the value of [`client_encoding`](https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-CLIENT-ENCODING).
