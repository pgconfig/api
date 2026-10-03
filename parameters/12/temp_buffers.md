---
name: "temp_buffers"
version: "12"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum number of temporary buffers used by each session."
context: "user"
unit: "8kB"
default: "1024"
min: "100"
max: "1073741823"
url: "https://www.postgresql.org/docs/12/runtime-config-resource.html#GUC-TEMP-BUFFERS"
---

Sets the maximum amount of memory used for temporary buffers within each database session. These are session-local buffers used only for access to temporary tables. If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default is eight megabytes (`8MB`). (If `BLCKSZ` is not 8kB, the default value scales proportionally to it.) This setting can be changed within individual sessions, but only before the first use of temporary tables within the session; subsequent attempts to change the value will have no effect on that session.

A session will allocate temporary buffers as needed up to the limit given by `temp_buffers`. The cost of setting a large value in sessions that do not actually need many temporary buffers is only a buffer descriptor, or about 64 bytes, per increment in `temp_buffers`. However if a buffer is actually used an additional 8192 bytes will be consumed for it (or in general, `BLCKSZ` bytes).
