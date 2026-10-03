---
name: "checkpoint_warning"
version: "17"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Sets the maximum time before warning if checkpoints triggered by WAL volume happen too frequently."
extra_desc: "Write a message to the server log if checkpoints caused by the filling of WAL segment files happen more frequently than this amount of time. Zero turns off the warning."
context: "sighup"
unit: "s"
default: "30"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-CHECKPOINT-WARNING"
---

Write a message to the server log if checkpoints caused by the filling of WAL segment files happen closer together than this amount of time (which suggests that `max_wal_size` ought to be raised). If this value is specified without units, it is taken as seconds. The default is 30 seconds (`30s`). Zero disables the warning. No warnings will be generated if `checkpoint_timeout` is less than `checkpoint_warning`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
