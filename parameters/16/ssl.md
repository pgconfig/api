---
name: "ssl"
version: "16"
type: "boolean"
category: "Connections and Authentication / SSL"
short_desc: "Enables SSL connections."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/16/runtime-config-connection.html#GUC-SSL"
---

Enables SSL connections. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `off`.
