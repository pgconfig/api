---
name: "promote_trigger_file"
version: "13"
type: "string"
category: "Replication / Standby Servers"
short_desc: "Specifies a file name whose presence ends recovery in the standby."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/13/runtime-config-replication.html#GUC-PROMOTE-TRIGGER-FILE"
---

Specifies a trigger file whose presence ends recovery in the standby. Even if this value is not set, you can still promote the standby using `pg_ctl promote` or calling `pg_promote()`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
