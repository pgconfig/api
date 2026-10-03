---
name: "bgwriter_lru_maxpages"
version: "9.6"
type: "integer"
category: "Resource Usage / Background Writer"
short_desc: "Background writer maximum number of LRU pages to flush per round."
context: "sighup"
default: "100"
min: "0"
max: "1000"
url: "https://www.postgresql.org/docs/9.6/runtime-config-resource.html#GUC-BGWRITER-LRU-MAXPAGES"
---

In each round, no more than this many buffers will be written by the background writer. Setting this to zero disables background writing. (Note that checkpoints, which are managed by a separate, dedicated auxiliary process, are unaffected.) The default value is 100 buffers. This parameter can only be set in the `postgresql.conf` file or on the server command line.
