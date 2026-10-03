---
name: "ssl_key_file"
version: "9.4"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Location of the SSL server private key file."
context: "postmaster"
default: "server.key"
url: "https://www.postgresql.org/docs/9.4/runtime-config-connection.html#GUC-SSL-KEY-FILE"
---

Specifies the name of the file containing the SSL server private key. The default is `server.key`. Relative paths are relative to the data directory. This parameter can only be set at server start.
