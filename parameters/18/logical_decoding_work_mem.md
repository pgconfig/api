---
name: "logical_decoding_work_mem"
version: "18"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum memory to be used for logical decoding."
extra_desc: "This much memory can be used by each internal reorder buffer before spilling to disk."
context: "user"
unit: "kB"
default: "65536"
min: "64"
max: "2147483647"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-LOGICAL-DECODING-WORK-MEM"
---

Specifies the maximum amount of memory to be used by logical decoding, before some of the decoded changes are written to local disk. This limits the amount of memory used by logical streaming replication connections. It defaults to 64 megabytes (`64MB`). Since each replication connection only uses a single buffer of this size, and an installation normally doesn't have many such connections concurrently (as limited by `max_wal_senders`), it's safe to set this value significantly higher than `work_mem`, reducing the amount of decoded changes written to disk.
