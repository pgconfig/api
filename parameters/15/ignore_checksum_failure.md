---
name: "ignore_checksum_failure"
version: "15"
type: "boolean"
category: "Developer Options"
short_desc: "Continues processing after a checksum failure."
extra_desc: "Detection of a checksum failure normally causes PostgreSQL to report an error, aborting the current transaction. Setting ignore_checksum_failure to true causes the system to ignore the failure (but still report a warning), and continue processing. This behavior could cause crashes or other serious problems. Only has an effect if checksums are enabled."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/15/runtime-config-developer.html#GUC-IGNORE-CHECKSUM-FAILURE"
---

Only has effect if [data checksums](https://www.postgresql.org/docs/15/app-initdb.html#APP-INITDB-DATA-CHECKSUMS) are enabled.

Detection of a checksum failure during a read normally causes PostgreSQL to report an error, aborting the current transaction. Setting `ignore_checksum_failure` to on causes the system to ignore the failure (but still report a warning), and continue processing. This behavior may *cause crashes, propagate or hide corruption, or other serious problems*. However, it may allow you to get past the error and retrieve undamaged tuples that might still be present in the table if the block header is still sane. If the header is corrupt an error will be reported even if this option is enabled. The default setting is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.
