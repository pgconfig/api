---
name: "temp_file_limit"
version: "19"
type: "integer"
category: "Resource Usage / Disk"
short_desc: "Limits the total size of all temporary files used by each process."
extra_desc: "-1 means no limit."
context: "superuser"
unit: "kB"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-TEMP-FILE-LIMIT"
---

Specifies the maximum amount of disk space that a process can use for temporary files, such as sort and hash temporary files, or the storage file for a held cursor. A transaction attempting to exceed this limit will be canceled. If this value is specified without units, it is taken as kilobytes. `-1` (the default) means no limit. Only superusers and users with the appropriate `SET` privilege can change this setting.

This setting constrains the total space used at any instant by all temporary files used by a given PostgreSQL process. It should be noted that disk space used for explicit temporary tables, as opposed to temporary files used behind-the-scenes in query execution, does *not* count against this limit.
