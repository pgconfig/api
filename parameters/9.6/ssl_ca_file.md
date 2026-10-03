---
name: "ssl_ca_file"
version: "9.6"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Location of the SSL certificate authority file."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/9.6/runtime-config-connection.html#GUC-SSL-CA-FILE"
---

Specifies the name of the file containing the SSL server certificate authority (CA). The default is empty, meaning no CA file is loaded, and client certificate verification is not performed. (In previous releases of PostgreSQL, the name of this file was hard-coded as `root.crt`.) Relative paths are relative to the data directory. This parameter can only be set at server start.
