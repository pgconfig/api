---
name: "log_temp_files"
version: "11"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "Log the use of temporary files larger than this number of kilobytes."
extra_desc: "Zero logs all files. The default is -1 (turning this feature off)."
context: "superuser"
unit: "kB"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/11/runtime-config-logging.html#GUC-LOG-TEMP-FILES"
---

Controls logging of temporary file names and sizes. Temporary files can be created for sorts, hashes, and temporary query results. A log entry is made for each temporary file when it is deleted. A value of zero logs all temporary file information, while positive values log only files whose size is greater than or equal to the specified number of kilobytes. The default setting is -1, which disables such logging. Only superusers can change this setting.
