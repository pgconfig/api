---
name: "max_connections"
version: "9.5"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the maximum number of concurrent connections."
context: "postmaster"
default: "100"
min: "1"
max: "8388607"
url: "https://www.postgresql.org/docs/9.5/runtime-config-connection.html#GUC-MAX-CONNECTIONS"
---

Determines the maximum number of concurrent connections to the database server. The default is typically 100 connections, but might be less if your kernel settings will not support it (as determined during initdb). This parameter can only be set at server start.

When running a standby server, you must set this parameter to the same or higher value than on the master server. Otherwise, queries will not be allowed in the standby server.
