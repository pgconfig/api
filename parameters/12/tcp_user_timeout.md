---
name: "tcp_user_timeout"
version: "12"
type: "integer"
category: "Client Connection Defaults / Other Defaults"
short_desc: "TCP user timeout."
extra_desc: "A value of 0 uses the system default."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/12/runtime-config-connection.html#GUC-TCP-USER-TIMEOUT"
---

Specifies the amount of time that transmitted data may remain unacknowledged before the TCP connection is forcibly closed. If this value is specified without units, it is taken as milliseconds. A value of 0 (the default) selects the operating system's default. This parameter is supported only on systems that support `TCP_USER_TIMEOUT`; on other systems, it must be zero. In sessions connected via a Unix-domain socket, this parameter is ignored and always reads as zero.

> [!NOTE]
> This parameter is not supported on Windows, and must be zero.
