---
name: "allow_in_place_tablespaces"
version: "19"
type: "boolean"
category: "Developer Options"
short_desc: "Allows tablespaces directly inside pg_tblspc, for testing."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-ALLOW-IN-PLACE-TABLESPACES"
---

Allows tablespaces to be created as directories inside `pg_tblspc`, when an empty location string is provided to the `CREATE TABLESPACE` command. This is intended to allow testing replication scenarios where primary and standby servers are running on the same machine. Such directories are likely to confuse backup tools that expect to find only symbolic links in that location. Only superusers and users with the appropriate `SET` privilege can change this setting.
