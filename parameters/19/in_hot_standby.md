---
name: "in_hot_standby"
version: "19"
type: "boolean"
category: "Preset Options"
short_desc: "Shows whether hot standby is currently active."
context: "internal"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-IN-HOT-STANDBY"
---

Reports whether the server is currently in hot standby mode. When this is `on`, all transactions are forced to be read-only. Within a session, this can change only if the server is promoted to be primary. See [Hot Standby](https://www.postgresql.org/docs/19/hot-standby.html) for more information.
