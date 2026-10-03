---
name: "update_process_title"
version: "10"
type: "boolean"
category: "Process Title"
short_desc: "Updates the process title to show the active SQL command."
extra_desc: "Enables updating of the process title every time a new SQL command is received by the server."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/10/runtime-config-logging.html#GUC-UPDATE-PROCESS-TITLE"
---

Enables updating of the process title every time a new SQL command is received by the server. This setting defaults to `on` on most platforms, but it defaults to `off` on Windows due to that platform's larger overhead for updating the process title. Only superusers can change this setting.
