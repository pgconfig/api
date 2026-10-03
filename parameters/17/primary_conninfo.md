---
name: "primary_conninfo"
version: "17"
type: "string"
category: "Replication / Standby Servers"
short_desc: "Sets the connection string to be used to connect to the sending server."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-PRIMARY-CONNINFO"
---

Specifies a connection string to be used for the standby server to connect with a sending server. This string is in the format described in [Connection Strings](https://www.postgresql.org/docs/17/libpq-connect.html#LIBPQ-CONNSTRING). If any option is unspecified in this string, then the corresponding environment variable (see [Environment Variables](https://www.postgresql.org/docs/17/libpq-envars.html)) is checked. If the environment variable is not set either, then defaults are used.

The connection string should specify the host name (or address) of the sending server, as well as the port number if it is not the same as the standby server's default. Also specify a user name corresponding to a suitably-privileged role on the sending server (see [Authentication](https://www.postgresql.org/docs/17/warm-standby.html#STREAMING-REPLICATION-AUTHENTICATION)). A password needs to be provided too, if the sender demands password authentication. It can be provided in the `primary_conninfo` string, or in a separate `~/.pgpass` file on the standby server (use `replication` as the database name).

For replication slot synchronization (see [Replication Slot Synchronization](https://www.postgresql.org/docs/17/logicaldecoding-explanation.html#LOGICALDECODING-REPLICATION-SLOTS-SYNCHRONIZATION)), it is also necessary to specify a valid `dbname` in the `primary_conninfo` string. This will only be used for slot synchronization. It is ignored for streaming.

This parameter can only be set in the `postgresql.conf` file or on the server command line. If this parameter is changed while the WAL receiver process is running, that process is signaled to shut down and expected to restart with the new setting (except if `primary_conninfo` is an empty string). This setting has no effect if the server is not in standby mode.
