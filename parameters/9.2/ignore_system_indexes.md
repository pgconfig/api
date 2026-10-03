---
name: "ignore_system_indexes"
version: "9.2"
type: "boolean"
category: "Developer Options"
short_desc: "Disables reading from system indexes."
extra_desc: "It does not prevent updating the indexes, so it is safe to use.  The worst consequence is slowness."
context: "backend"
default: "off"
url: "https://www.postgresql.org/docs/9.2/runtime-config-developer.html#GUC-IGNORE-SYSTEM-INDEXES"
---

Ignore system indexes when reading system tables (but still update the indexes when modifying the tables). This is useful when recovering from damaged system indexes. This parameter cannot be changed after session start.
