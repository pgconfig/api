---
name: "checkpoint_timeout"
version: "14"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Sets the maximum time between automatic WAL checkpoints."
context: "sighup"
unit: "s"
default: "300"
min: "30"
max: "86400"
url: "https://www.postgresql.org/docs/14/runtime-config-wal.html#GUC-CHECKPOINT-TIMEOUT"
---

Maximum time between automatic WAL checkpoints. If this value is specified without units, it is taken as seconds. The valid range is between 30 seconds and one day. The default is five minutes (`5min`). Increasing this parameter can increase the amount of time needed for crash recovery. This parameter can only be set in the `postgresql.conf` file or on the server command line.
