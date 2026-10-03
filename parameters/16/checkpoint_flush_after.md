---
name: "checkpoint_flush_after"
version: "16"
type: "integer"
category: "Write-Ahead Log / Checkpoints"
short_desc: "Number of pages after which previously performed writes are flushed to disk."
context: "sighup"
unit: "8kB"
default: "32"
min: "0"
max: "256"
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-CHECKPOINT-FLUSH-AFTER"
---

Whenever more than this amount of data has been written while performing a checkpoint, attempt to force the OS to issue these writes to the underlying storage. Doing so will limit the amount of dirty data in the kernel's page cache, reducing the likelihood of stalls when an `fsync` is issued at the end of the checkpoint, or when the OS writes data back in larger batches in the background. Often that will result in greatly reduced transaction latency, but there also are some cases, especially with workloads that are bigger than [`shared_buffers`](https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-SHARED-BUFFERS), but smaller than the OS's page cache, where performance might degrade. This setting may have no effect on some platforms. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The valid range is between `0`, which disables forced writeback, and `2MB`. The default is `256kB` on Linux, `0` elsewhere. (If `BLCKSZ` is not 8kB, the default and maximum values scale proportionally to it.) This parameter can only be set in the `postgresql.conf` file or on the server command line.
