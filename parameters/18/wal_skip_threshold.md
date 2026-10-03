---
name: "wal_skip_threshold"
version: "18"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Minimum size of new file to fsync instead of writing WAL."
context: "user"
unit: "kB"
default: "2048"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-WAL-SKIP-THRESHOLD"
---

When `wal_level` is `minimal` and a transaction commits after creating or rewriting a permanent relation, this setting determines how to persist the new data. If the data is smaller than this setting, write it to the WAL log; otherwise, use an fsync of affected files. Depending on the properties of your storage, raising or lowering this value might help if such commits are slowing concurrent transactions. If this value is specified without units, it is taken as kilobytes. The default is two megabytes (`2MB`).
