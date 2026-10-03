---
name: "min_wal_size"
version: "9.5"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Sets the minimum size to shrink the WAL to."
context: "sighup"
unit: "16MB"
default: "5"
min: "2"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.5/runtime-config-wal.html#GUC-MIN-WAL-SIZE"
---

As long as WAL disk usage stays below this setting, old WAL files are always recycled for future use at a checkpoint, rather than removed. This can be used to ensure that enough WAL space is reserved to handle spikes in WAL usage, for example when running large batch jobs. The default is 80 MB. This parameter can only be set in the `postgresql.conf` file or on the server command line.
