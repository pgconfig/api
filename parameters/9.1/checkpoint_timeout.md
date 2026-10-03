---
name: "checkpoint_timeout"
version: "9.1"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Sets the maximum time between automatic WAL checkpoints."
context: "sighup"
unit: "s"
default: "300"
min: "30"
max: "3600"
url: "https://www.postgresql.org/docs/9.1/runtime-config-wal.html#GUC-CHECKPOINT-TIMEOUT"
---

Maximum time between automatic WAL checkpoints, in seconds. The default is five minutes (`5min`). Increasing this parameter can increase the amount of time needed for crash recovery. This parameter can only be set in the `postgresql.conf` file or on the server command line.
