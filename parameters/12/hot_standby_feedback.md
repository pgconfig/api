---
name: "hot_standby_feedback"
version: "12"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Allows feedback from a hot standby to the primary that will avoid query conflicts."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/12/runtime-config-replication.html#GUC-HOT-STANDBY-FEEDBACK"
---

Specifies whether or not a hot standby will send feedback to the primary or upstream standby about queries currently executing on the standby. This parameter can be used to eliminate query cancels caused by cleanup records, but can cause database bloat on the primary for some workloads. Feedback messages will not be sent more frequently than once per `wal_receiver_status_interval`. The default value is `off`. This parameter can only be set in the `postgresql.conf` file or on the server command line.

If cascaded replication is in use the feedback is passed upstream until it eventually reaches the primary. Standbys make no other use of feedback they receive other than to pass upstream.

This setting does not override the behavior of `old_snapshot_threshold` on the primary; a snapshot on the standby which exceeds the primary's age threshold can become invalid, resulting in cancellation of transactions on the standby. This is because `old_snapshot_threshold` is intended to provide an absolute limit on the time which dead rows can contribute to bloat, which would otherwise be violated because of the configuration of a standby.
