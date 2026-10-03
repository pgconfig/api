---
name: "max_connections"
version: "9.2"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the maximum number of concurrent connections."
context: "postmaster"
default: "100"
min: "1"
max: "8388607"
url: "https://www.postgresql.org/docs/9.2/runtime-config-connection.html#GUC-MAX-CONNECTIONS"
---

Determines the maximum number of concurrent connections to the database server. The default is typically 100 connections, but might be less if your kernel settings will not support it (as determined during initdb). This parameter can only be set at server start.

Increasing this parameter might cause PostgreSQL to request more `System V` shared memory or semaphores than your operating system's default configuration allows. See [Shared Memory and Semaphores](https://www.postgresql.org/docs/9.2/kernel-resources.html#SYSVIPC) for information on how to adjust those parameters, if necessary.

When running a standby server, you must set this parameter to the same or higher value than on the master server. Otherwise, queries will not be allowed in the standby server.
