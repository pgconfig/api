---
name: "update_process_title"
version: "9.5"
type: "boolean"
category: "Process Title"
short_desc: "Updates the process title to show the active SQL command."
extra_desc: "Enables updating of the process title every time a new SQL command is received by the server."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/9.5/runtime-config-logging.html#GUC-UPDATE-PROCESS-TITLE"
---

Enables updating of the process title every time a new SQL command is received by the server. The process title is typically viewed by the `ps` command, or in Windows by using the Process Explorer. Only superusers can change this setting.
