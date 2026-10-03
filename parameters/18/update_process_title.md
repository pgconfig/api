---
name: "update_process_title"
version: "18"
type: "boolean"
category: "Reporting and Logging / Process Title"
short_desc: "Updates the process title to show the active SQL command."
extra_desc: "Enables updating of the process title every time a new SQL command is received by the server."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-UPDATE-PROCESS-TITLE"
---

Enables updating of the process title every time a new SQL command is received by the server. This setting defaults to `on` on most platforms, but it defaults to `off` on Windows due to that platform's larger overhead for updating the process title. Only superusers and users with the appropriate `SET` privilege can change this setting.
