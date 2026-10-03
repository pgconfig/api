---
name: "ssl_library"
version: "12"
type: "string"
category: "Preset Options"
short_desc: "Name of the SSL library."
context: "internal"
default: "OpenSSL"
url: "https://www.postgresql.org/docs/12/runtime-config-preset.html#GUC-SSL-LIBRARY"
---

Reports the name of the SSL library that this PostgreSQL server was built with (even if SSL is not currently configured or in use on this instance), for example `OpenSSL`, or an empty string if none.
