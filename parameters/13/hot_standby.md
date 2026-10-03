---
name: "hot_standby"
version: "13"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Allows connections and queries during recovery."
context: "postmaster"
default: "on"
url: "https://www.postgresql.org/docs/13/runtime-config-replication.html#GUC-HOT-STANDBY"
---

Specifies whether or not you can connect and run queries during recovery, as described in [Hot Standby](https://www.postgresql.org/docs/13/hot-standby.html). The default value is `on`. This parameter can only be set at server start. It only has effect during archive recovery or in standby mode.
