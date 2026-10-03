---
name: "authentication_timeout"
version: "9.4"
type: "integer"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Sets the maximum allowed time to complete client authentication."
context: "sighup"
unit: "s"
default: "60"
min: "1"
max: "600"
url: "https://www.postgresql.org/docs/9.4/runtime-config-connection.html#GUC-AUTHENTICATION-TIMEOUT"
---

Maximum time to complete client authentication, in seconds. If a would-be client has not completed the authentication protocol in this much time, the server closes the connection. This prevents hung clients from occupying a connection indefinitely. The default is one minute (`1m`). This parameter can only be set in the `postgresql.conf` file or on the server command line.
