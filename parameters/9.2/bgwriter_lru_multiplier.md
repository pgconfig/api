---
name: "bgwriter_lru_multiplier"
version: "9.2"
type: "floating point"
category: "Resource Usage / Background Writer"
short_desc: "Multiple of the average buffer usage to free per round."
context: "sighup"
default: "2"
min: "0"
max: "10"
url: "https://www.postgresql.org/docs/9.2/runtime-config-resource.html#GUC-BGWRITER-LRU-MULTIPLIER"
---

The number of dirty buffers written in each round is based on the number of new buffers that have been needed by server processes during recent rounds. The average recent need is multiplied by `bgwriter_lru_multiplier` to arrive at an estimate of the number of buffers that will be needed during the next round. Dirty buffers are written until there are that many clean, reusable buffers available. (However, no more than `bgwriter_lru_maxpages` buffers will be written per round.) Thus, a setting of 1.0 represents a "just in time" policy of writing exactly the number of buffers predicted to be needed. Larger values provide some cushion against spikes in demand, while smaller values intentionally leave writes to be done by server processes. The default is 2.0. This parameter can only be set in the `postgresql.conf` file or on the server command line.
