---
name: "wal_buffers"
version: "10"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the number of disk-page buffers in shared memory for WAL."
context: "postmaster"
unit: "8kB"
default: "-1"
min: "-1"
max: "262143"
url: "https://www.postgresql.org/docs/10/runtime-config-wal.html#GUC-WAL-BUFFERS"
---

The amount of shared memory used for WAL data that has not yet been written to disk. The default setting of -1 selects a size equal to 1/32nd (about 3%) of [`shared_buffers`](https://www.postgresql.org/docs/10/runtime-config-resource.html#GUC-SHARED-BUFFERS), but not less than `64kB` nor more than the size of one WAL segment, typically `16MB`. This value can be set manually if the automatic choice is too large or too small, but any positive value less than `32kB` will be treated as `32kB`. This parameter can only be set at server start.

The contents of the WAL buffers are written out to disk at every transaction commit, so extremely large values are unlikely to provide a significant benefit. However, setting this value to at least a few megabytes can improve write performance on a busy server where many clients are committing at once. The auto-tuning selected by the default setting of -1 should give reasonable results in most cases.
