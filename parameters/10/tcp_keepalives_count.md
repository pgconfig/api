---
name: "tcp_keepalives_count"
version: "10"
type: "integer"
category: "Client Connection Defaults / Other Defaults"
short_desc: "Maximum number of TCP keepalive retransmits."
extra_desc: "This controls the number of consecutive keepalive retransmits that can be lost before a connection is considered dead. A value of 0 uses the system default."
context: "user"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/10/runtime-config-connection.html#GUC-TCP-KEEPALIVES-COUNT"
---

Specifies the number of TCP keepalives that can be lost before the server's connection to the client is considered dead. A value of 0 uses the system default. This parameter is supported only on systems that support `TCP_KEEPCNT` or an equivalent socket option; on other systems, it must be zero. In sessions connected via a Unix-domain socket, this parameter is ignored and always reads as zero.

> [!NOTE]
> This parameter is not supported on Windows, and must be zero.
