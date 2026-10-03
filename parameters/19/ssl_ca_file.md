---
name: "ssl_ca_file"
version: "19"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Location of the SSL certificate authority file."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-SSL-CA-FILE"
---

Specifies the name of the file containing the SSL server certificate authority (CA). Relative paths are relative to the data directory. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is empty, meaning no CA file is loaded, and client certificate verification is not performed.
