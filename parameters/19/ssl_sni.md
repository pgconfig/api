---
name: "ssl_sni"
version: "19"
type: "boolean"
category: "Connections and Authentication / SSL"
short_desc: "Sets whether to interpret SNI extensions in SSL connections."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-SSL-SNI"
---

Enables SNI configuration for SSL connections. When set to `on` host configuration from [`hosts_file`](https://www.postgresql.org/docs/19/runtime-config-file-locations.html#GUC-HOSTS-FILE) is used, see [SNI Configuration](https://www.postgresql.org/docs/19/ssl-tcp.html#SSL-SNI) for more details.

This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `off`.
