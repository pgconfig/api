---
name: "ssl_key_file"
version: "16"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Location of the SSL server private key file."
context: "sighup"
default: "server.key"
url: "https://www.postgresql.org/docs/16/runtime-config-connection.html#GUC-SSL-KEY-FILE"
---

Specifies the name of the file containing the SSL server private key. Relative paths are relative to the data directory. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `server.key`.
