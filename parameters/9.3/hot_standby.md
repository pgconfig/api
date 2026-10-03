---
name: "hot_standby"
version: "9.3"
type: "boolean"
category: "Replication / Standby Servers"
short_desc: "Allows connections and queries during recovery."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/9.3/runtime-config-replication.html#GUC-HOT-STANDBY"
---

Specifies whether or not you can connect and run queries during recovery, as described in [Hot Standby](https://www.postgresql.org/docs/9.3/hot-standby.html). The default value is `off`. This parameter can only be set at server start. It only has effect during archive recovery or in standby mode.
