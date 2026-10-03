---
name: "checkpoint_segments"
version: "9.1"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Sets the maximum distance in log segments between automatic WAL checkpoints."
context: "sighup"
default: "3"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.1/runtime-config-wal.html#GUC-CHECKPOINT-SEGMENTS"
---

Maximum number of log file segments between automatic WAL checkpoints (each segment is normally 16 megabytes). The default is three segments. Increasing this parameter can increase the amount of time needed for crash recovery. This parameter can only be set in the `postgresql.conf` file or on the server command line.
