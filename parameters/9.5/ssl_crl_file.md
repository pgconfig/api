---
name: "ssl_crl_file"
version: "9.5"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Location of the SSL certificate revocation list file."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/9.5/runtime-config-connection.html#GUC-SSL-CRL-FILE"
---

Specifies the name of the file containing the SSL server certificate revocation list (CRL). The default is empty, meaning no CRL file is loaded. (In previous releases of PostgreSQL, the name of this file was hard-coded as `root.crl`.) Relative paths are relative to the data directory. This parameter can only be set at server start.
