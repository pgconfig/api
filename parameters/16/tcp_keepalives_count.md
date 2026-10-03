---
name: "tcp_keepalives_count"
version: "16"
type: "integer"
category: "Connections and Authentication / TCP Settings"
short_desc: "Maximum number of TCP keepalive retransmits."
extra_desc: "Number of consecutive keepalive retransmits that can be lost before a connection is considered dead. A value of 0 uses the system default."
context: "user"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-connection.html#GUC-TCP-KEEPALIVES-COUNT"
---

Specifies the number of TCP keepalive messages that can be lost before the server's connection to the client is considered dead. A value of 0 (the default) selects the operating system's default. This parameter is supported only on systems that support `TCP_KEEPCNT` or an equivalent socket option (which does not include Windows); on other systems, it must be zero. In sessions connected via a Unix-domain socket, this parameter is ignored and always reads as zero.
