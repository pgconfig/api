---
name: "checkpoint_warning"
version: "12"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Enables warnings if checkpoint segments are filled more frequently than this."
extra_desc: "Write a message to the server log if checkpoints caused by the filling of checkpoint segment files happens more frequently than this number of seconds. Zero turns off the warning."
context: "sighup"
unit: "s"
default: "30"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/12/runtime-config-wal.html#GUC-CHECKPOINT-WARNING"
---

Write a message to the server log if checkpoints caused by the filling of WAL segment files happen closer together than this amount of time (which suggests that `max_wal_size` ought to be raised). If this value is specified without units, it is taken as seconds. The default is 30 seconds (`30s`). Zero disables the warning. No warnings will be generated if `checkpoint_timeout` is less than `checkpoint_warning`. This parameter can only be set in the `postgresql.conf` file or on the server command line.
