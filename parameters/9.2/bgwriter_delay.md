---
name: "bgwriter_delay"
version: "9.2"
type: "integer"
category: "Resource Usage / Background Writer"
short_desc: "Background writer sleep time between rounds."
context: "sighup"
unit: "ms"
default: "200"
min: "10"
max: "10000"
url: "https://www.postgresql.org/docs/9.2/runtime-config-resource.html#GUC-BGWRITER-DELAY"
---

Specifies the delay between activity rounds for the background writer. In each round the writer issues writes for some number of dirty buffers (controllable by the following parameters). It then sleeps for `bgwriter_delay` milliseconds, and repeats. When there are no dirty buffers in the buffer pool, though, it goes into a longer sleep regardless of `bgwriter_delay`. The default value is 200 milliseconds (`200ms`). Note that on many systems, the effective resolution of sleep delays is 10 milliseconds; setting `bgwriter_delay` to a value that is not a multiple of 10 might have the same results as setting it to the next higher multiple of 10. This parameter can only be set in the `postgresql.conf` file or on the server command line.
