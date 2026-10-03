---
name: "client_connection_check_interval"
version: "19"
type: "integer"
category: "Connections and Authentication / TCP Settings"
short_desc: "Sets the time interval between checks for disconnection while running queries."
extra_desc: "0 disables connection checks."
context: "user"
unit: "ms"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-CLIENT-CONNECTION-CHECK-INTERVAL"
---

Sets the time interval between optional checks that the client is still connected, while running queries. The check is performed by polling the socket, and allows long running queries to be aborted sooner if the kernel reports that the connection is closed.

This option relies on kernel events exposed by Linux, macOS, illumos and the BSD family of operating systems, and is not currently available on other systems.

If the value is specified without units, it is taken as milliseconds. The default value is `0`, which disables connection checks. Without connection checks, the server will detect the loss of the connection only at the next interaction with the socket, when it waits for, receives or sends data.

For the kernel itself to detect lost TCP connections reliably and within a known timeframe in all scenarios including network failure, it may also be necessary to adjust the TCP keepalive settings of the operating system, or the [`tcp_keepalives_idle`](https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-TCP-KEEPALIVES-IDLE), [`tcp_keepalives_interval`](https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-TCP-KEEPALIVES-INTERVAL) and [`tcp_keepalives_count`](https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-TCP-KEEPALIVES-COUNT) settings of PostgreSQL.
