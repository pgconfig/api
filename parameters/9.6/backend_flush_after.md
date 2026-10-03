---
name: "backend_flush_after"
version: "9.6"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Number of pages after which previously performed writes are flushed to disk."
context: "user"
unit: "8kB"
default: "0"
min: "0"
max: "256"
url: "https://www.postgresql.org/docs/9.6/runtime-config-resource.html#GUC-BACKEND-FLUSH-AFTER"
---

Whenever more than `backend_flush_after` bytes have been written by a single backend, attempt to force the OS to issue these writes to the underlying storage. Doing so will limit the amount of dirty data in the kernel's page cache, reducing the likelihood of stalls when an fsync is issued at the end of a checkpoint, or when the OS writes data back in larger batches in the background. Often that will result in greatly reduced transaction latency, but there also are some cases, especially with workloads that are bigger than [`shared_buffers`](https://www.postgresql.org/docs/9.6/runtime-config-resource.html#GUC-SHARED-BUFFERS), but smaller than the OS's page cache, where performance might degrade. This setting may have no effect on some platforms. The valid range is between `0`, which disables forced writeback, and `2MB`. The default is `0`, i.e., no forced writeback. (If `BLCKSZ` is not 8kB, the maximum value scales proportionally to it.)
