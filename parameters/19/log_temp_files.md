---
name: "log_temp_files"
version: "19"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "Log the use of temporary files larger than this number of kilobytes."
extra_desc: "-1 disables logging temporary files. 0 means log all temporary files."
context: "superuser"
unit: "kB"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-TEMP-FILES"
---

Controls logging of temporary file names and sizes. Temporary files can be created for sorts, hashes, and temporary query results. If enabled by this setting, a log entry is emitted for each temporary file, with the file size specified in bytes, when it is deleted. A value of zero logs all temporary file information, while positive values log only files whose size is greater than or equal to the specified amount of data. If this value is specified without units, it is taken as kilobytes. The default setting is -1, which disables such logging. Only superusers and users with the appropriate `SET` privilege can change this setting.
