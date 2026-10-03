---
name: "allow_system_table_mods"
version: "17"
type: "boolean"
category: "Developer Options"
short_desc: "Allows modifications of the structure of system tables."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/17/runtime-config-developer.html#GUC-ALLOW-SYSTEM-TABLE-MODS"
---

Allows modification of the structure of system tables as well as certain other risky actions on system tables. This is otherwise not allowed even for superusers. Ill-advised use of this setting can cause irretrievable data loss or seriously corrupt the database system. Only superusers and users with the appropriate `SET` privilege can change this setting.
