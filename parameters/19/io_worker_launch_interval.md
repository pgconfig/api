---
name: "io_worker_launch_interval"
version: "19"
type: "integer"
category: "Resource Usage / I/O"
short_desc: "Minimum time before launching a new I/O worker process, for io_method=worker."
context: "sighup"
unit: "ms"
default: "100"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-WORKER-LAUNCH-INTERVAL"
---

Sets the minimum time before another I/O worker can be launched. This avoids creating too many for an unsustained burst of activity. The default is 100ms. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Only has an effect if [`io_method`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-METHOD) is set to `worker`.
