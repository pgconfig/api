---
name: "io_max_workers"
version: "19"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Maximum number of I/O worker processes, for io_method=worker."
context: "sighup"
default: "8"
min: "1"
max: "32"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-MAX-WORKERS"
---

Sets the maximum number of I/O worker processes. The default is 8. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Only has an effect if [`io_method`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-METHOD) is set to `worker`.
