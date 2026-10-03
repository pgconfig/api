---
name: "ignore_invalid_pages"
version: "16"
type: "boolean"
category: "Developer Options"
short_desc: "Continues recovery after an invalid pages failure."
extra_desc: "Detection of WAL records having references to invalid pages during recovery causes PostgreSQL to raise a PANIC-level error, aborting the recovery. Setting ignore_invalid_pages to true causes the system to ignore invalid page references in WAL records (but still report a warning), and continue recovery. This behavior may cause crashes, data loss, propagate or hide corruption, or other serious problems. Only has an effect during recovery or in standby mode."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/16/runtime-config-developer.html#GUC-IGNORE-INVALID-PAGES"
---

If set to `off` (the default), detection of WAL records having references to invalid pages during recovery causes PostgreSQL to raise a PANIC-level error, aborting the recovery. Setting `ignore_invalid_pages` to `on` causes the system to ignore invalid page references in WAL records (but still report a warning), and continue the recovery. This behavior may *cause crashes, data loss, propagate or hide corruption, or other serious problems*. However, it may allow you to get past the PANIC-level error, to finish the recovery, and to cause the server to start up. The parameter can only be set at server start. It only has effect during recovery or in standby mode.
