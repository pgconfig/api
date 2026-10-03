---
name: "ssl_crl_file"
version: "15"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Location of the SSL certificate revocation list file."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/15/runtime-config-connection.html#GUC-SSL-CRL-FILE"
---

Specifies the name of the file containing the SSL client certificate revocation list (CRL). Relative paths are relative to the data directory. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is empty, meaning no CRL file is loaded (unless [`ssl_crl_dir`](https://www.postgresql.org/docs/15/runtime-config-connection.html#GUC-SSL-CRL-DIR) is set).
