---
name: "tcp_keepalives_idle"
version: "9.1"
type: "integer"
category: "Client Connection Defaults / Other Defaults"
short_desc: "Time between issuing TCP keepalives."
extra_desc: "A value of 0 uses the system default."
context: "user"
unit: "s"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.1/runtime-config-connection.html#GUC-TCP-KEEPALIVES-IDLE"
---

Specifies the number of seconds before sending a keepalive packet on an otherwise idle connection. A value of 0 uses the system default. This parameter is supported only on systems that support the `TCP_KEEPIDLE` or `TCP_KEEPALIVE` symbols, and on Windows; on other systems, it must be zero. In sessions connected via a Unix-domain socket, this parameter is ignored and always reads as zero.

> [!NOTE]
> On Windows, a value of 0 will set this parameter to 2 hours, since Windows does not provide a way to read the system default value.
