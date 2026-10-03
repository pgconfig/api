---
name: "remove_temp_files_after_crash"
version: "17"
type: "boolean"
category: "Developer Options"
short_desc: "Remove temporary files after backend crash."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/17/runtime-config-developer.html#GUC-REMOVE-TEMP-FILES-AFTER-CRASH"
---

When set to `on`, which is the default, PostgreSQL will automatically remove temporary files after a backend crash. If disabled, the files will be retained and may be used for debugging, for example. Repeated crashes may however result in accumulation of useless files. This parameter can only be set in the `postgresql.conf` file or on the server command line.
