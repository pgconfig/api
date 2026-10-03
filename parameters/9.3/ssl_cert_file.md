---
name: "ssl_cert_file"
version: "9.3"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Location of the SSL server certificate file."
context: "postmaster"
default: "server.crt"
url: "https://www.postgresql.org/docs/9.3/runtime-config-connection.html#GUC-SSL-CERT-FILE"
---

Specifies the name of the file containing the SSL server certificate. The default is `server.crt`. Relative paths are relative to the data directory. This parameter can only be set at server start.
