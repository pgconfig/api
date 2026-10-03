---
name: "allow_system_table_mods"
version: "11"
type: "boolean"
category: "Developer Options"
short_desc: "Allows modifications of the structure of system tables."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/11/runtime-config-developer.html#GUC-ALLOW-SYSTEM-TABLE-MODS"
---

Allows modification of the structure of system tables. This is used by `initdb`. This parameter can only be set at server start.
