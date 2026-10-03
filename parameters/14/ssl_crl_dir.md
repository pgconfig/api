---
name: "ssl_crl_dir"
version: "14"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Location of the SSL certificate revocation list directory."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/14/runtime-config-connection.html#GUC-SSL-CRL-DIR"
---

Specifies the name of the directory containing the SSL client certificate revocation list (CRL). Relative paths are relative to the data directory. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is empty, meaning no CRLs are used (unless [`ssl_crl_file`](https://www.postgresql.org/docs/14/runtime-config-connection.html#GUC-SSL-CRL-FILE) is set).

The directory needs to be prepared with the OpenSSL command `openssl rehash` or `c_rehash`. See its documentation for details.

When using this setting, CRLs in the specified directory are loaded on-demand at connection time. New CRLs can be added to the directory and will be used immediately. This is unlike [`ssl_crl_file`](https://www.postgresql.org/docs/14/runtime-config-connection.html#GUC-SSL-CRL-FILE), which causes the CRL in the file to be loaded at server start time or when the configuration is reloaded. Both settings can be used together.
