---
name: "checkpoint_completion_target"
version: "9.6"
type: "floating point"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Time spent flushing dirty buffers during checkpoint, as fraction of checkpoint interval."
context: "sighup"
default: "0.5"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/9.6/runtime-config-wal.html#GUC-CHECKPOINT-COMPLETION-TARGET"
---

Specifies the target of checkpoint completion, as a fraction of total time between checkpoints. The default is 0.5. This parameter can only be set in the `postgresql.conf` file or on the server command line.
