---
name: "checkpoint_completion_target"
version: "15"
type: "floating point"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Time spent flushing dirty buffers during checkpoint, as fraction of checkpoint interval."
context: "sighup"
default: "0.9"
min: "0"
max: "1"
url: "https://www.postgresql.org/docs/15/runtime-config-wal.html#GUC-CHECKPOINT-COMPLETION-TARGET"
---

Specifies the target of checkpoint completion, as a fraction of total time between checkpoints. The default is 0.9, which spreads the checkpoint across almost all of the available interval, providing fairly consistent I/O load while also leaving some time for checkpoint completion overhead. Reducing this parameter is not recommended because it causes the checkpoint to complete faster. This results in a higher rate of I/O during the checkpoint followed by a period of less I/O between the checkpoint completion and the next scheduled checkpoint. This parameter can only be set in the `postgresql.conf` file or on the server command line.
