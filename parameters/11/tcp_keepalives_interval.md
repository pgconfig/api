---
name: "tcp_keepalives_interval"
version: "11"
type: "integer"
category: "Client Connection Defaults / Other Defaults"
short_desc: "Time between TCP keepalive retransmits."
extra_desc: "A value of 0 uses the system default."
context: "user"
unit: "s"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/11/runtime-config-connection.html#GUC-TCP-KEEPALIVES-INTERVAL"
---

Specifies the number of seconds after which a TCP keepalive message that is not acknowledged by the client should be retransmitted. A value of 0 uses the system default. This parameter is supported only on systems that support `TCP_KEEPINTVL` or an equivalent socket option, and on Windows; on other systems, it must be zero. In sessions connected via a Unix-domain socket, this parameter is ignored and always reads as zero.

> [!NOTE]
> On Windows, a value of 0 will set this parameter to 1 second, since Windows does not provide a way to read the system default value.
