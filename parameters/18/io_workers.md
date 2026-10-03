---
name: "io_workers"
version: "18"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Number of IO worker processes, for io_method=worker."
context: "sighup"
default: "3"
min: "1"
max: "32"
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-IO-WORKERS"
---

Selects the number of I/O worker processes to use. The default is 3. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Only has an effect if [`io_method`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-IO-METHOD) is set to `worker`.
