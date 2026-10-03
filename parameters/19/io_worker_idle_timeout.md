---
name: "io_worker_idle_timeout"
version: "19"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Maximum time before idle I/O worker processes time out, for io_method=worker."
context: "sighup"
unit: "ms"
default: "60000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-WORKER-IDLE-TIMEOUT"
---

Sets the time after which entirely idle I/O worker processes exit, reducing the size of pool to match demand. The default is 1 minute. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Only has an effect if [`io_method`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-METHOD) is set to `worker`.
