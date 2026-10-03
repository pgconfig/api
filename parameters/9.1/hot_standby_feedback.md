---
name: "hot_standby_feedback"
version: "9.1"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Allows feedback from a hot standby to the primary that will avoid query conflicts."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/9.1/runtime-config-replication.html#GUC-HOT-STANDBY-FEEDBACK"
---

Specifies whether or not a hot standby will send feedback to the primary about queries currently executing on the standby. This parameter can be used to eliminate query cancels caused by cleanup records, but can cause database bloat on the primary for some workloads. Feedback messages will not be sent more frequently than once per `wal_receiver_status_interval`. The default value is `off`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
