---
name: "max_wal_size"
version: "13"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Sets the WAL size that triggers a checkpoint."
context: "sighup"
unit: "MB"
default: "1024"
min: "2"
max: "2147483647"
url: "https://www.postgresql.org/docs/13/runtime-config-wal.html#GUC-MAX-WAL-SIZE"
---

Maximum size to let the WAL grow during automatic checkpoints. This is a soft limit; WAL size can exceed `max_wal_size` under special circumstances, such as heavy load, a failing `archive_command`, or a high `wal_keep_size` setting. If this value is specified without units, it is taken as megabytes. The default is 1 GB. Increasing this parameter can increase the amount of time needed for crash recovery. This parameter can only be set in the `postgresql.conf` file or on the server command line.
