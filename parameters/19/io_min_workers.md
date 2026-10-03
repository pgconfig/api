---
name: "io_min_workers"
version: "19"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Minimum number of I/O worker processes, for io_method=worker."
context: "sighup"
default: "2"
min: "1"
max: "32"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-MIN-WORKERS"
---

Sets the minimum number of I/O worker processes. The default is 2. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Only has an effect if [`io_method`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-METHOD) is set to `worker`.
