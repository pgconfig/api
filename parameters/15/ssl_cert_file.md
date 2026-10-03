---
name: "ssl_cert_file"
version: "15"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Location of the SSL server certificate file."
context: "sighup"
default: "server.crt"
url: "https://www.postgresql.org/docs/15/runtime-config-connection.html#GUC-SSL-CERT-FILE"
---

Specifies the name of the file containing the SSL server certificate. Relative paths are relative to the data directory. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `server.crt`.
