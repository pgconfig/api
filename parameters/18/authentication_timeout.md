---
name: "authentication_timeout"
version: "18"
type: "integer"
category: "Connections and Authentication / Authentication"
short_desc: "Sets the maximum allowed time to complete client authentication."
context: "sighup"
unit: "s"
default: "60"
min: "1"
max: "600"
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-AUTHENTICATION-TIMEOUT"
---

Maximum amount of time allowed to complete client authentication. If a would-be client has not completed the authentication protocol in this much time, the server closes the connection. This prevents hung clients from occupying a connection indefinitely. If this value is specified without units, it is taken as seconds. The default is one minute (`1m`). This parameter can only be set in the `postgresql.conf` file or on the server command line.
