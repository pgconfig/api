---
name: "ssl"
version: "10"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Enables SSL connections."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/10/runtime-config-connection.html#GUC-SSL"
---

Enables SSL connections. Please read [Secure TCP/IP Connections with SSL](https://www.postgresql.org/docs/10/ssl-tcp.html) before using this. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `off`.
