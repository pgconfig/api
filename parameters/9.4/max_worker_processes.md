---
name: "max_worker_processes"
version: "9.4"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Maximum number of concurrent worker processes."
context: "postmaster"
default: "8"
min: "1"
max: "8388607"
url: "https://www.postgresql.org/docs/9.4/runtime-config-resource.html#GUC-MAX-WORKER-PROCESSES"
---

Sets the maximum number of background processes that the system can support. This parameter can only be set at server start.

When running a standby server, you must set this parameter to the same or higher value than on the master server. Otherwise, queries will not be allowed in the standby server.
