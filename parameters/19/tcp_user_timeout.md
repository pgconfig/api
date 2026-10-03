---
name: "tcp_user_timeout"
version: "19"
type: "integer"
category: "Connections and Authentication / TCP Settings"
short_desc: "TCP user timeout."
extra_desc: "0 means use the system default."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-TCP-USER-TIMEOUT"
---

Specifies the amount of time that transmitted data may remain unacknowledged before the TCP connection is forcibly closed. If this value is specified without units, it is taken as milliseconds. A value of 0 (the default) selects the operating system's default. This parameter is supported only on systems that support `TCP_USER_TIMEOUT` (which does not include Windows); on other systems, it must be zero. In sessions connected via a Unix-domain socket, this parameter is ignored and always reads as zero.
