---
name: "hot_standby_feedback"
version: "17"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Allows feedback from a hot standby to the primary that will avoid query conflicts."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-HOT-STANDBY-FEEDBACK"
---

Specifies whether or not a hot standby will send feedback to the primary or upstream standby about queries currently executing on the standby. This parameter can be used to eliminate query cancels caused by cleanup records, but can cause database bloat on the primary for some workloads. Feedback messages will not be sent more frequently than once per `wal_receiver_status_interval`. The default value is `off`. This parameter can only be set in the `postgresql.conf` file or on the server command line.

If cascaded replication is in use the feedback is passed upstream until it eventually reaches the primary. Standbys make no other use of feedback they receive other than to pass upstream.
