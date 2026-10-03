---
name: "ssl"
version: "9.3"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Enables SSL connections."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/9.3/runtime-config-connection.html#GUC-SSL"
---

Enables SSL connections. Please read [Secure TCP/IP Connections with SSL](https://www.postgresql.org/docs/9.3/ssl-tcp.html) before using this. The default is `off`. This parameter can only be set at server start. SSL communication is only possible with TCP/IP connections.
