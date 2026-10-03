---
name: "vacuum_buffer_usage_limit"
version: "19"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the buffer pool size for VACUUM, ANALYZE, and autovacuum."
context: "user"
unit: "kB"
default: "2048"
min: "0"
max: "16777216"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-VACUUM-BUFFER-USAGE-LIMIT"
---

Specifies the size of the [Buffer Access Strategy](https://www.postgresql.org/docs/19/glossary.html#GLOSSARY-BUFFER-ACCESS-STRATEGY) used by the `VACUUM` and `ANALYZE` commands. A setting of `0` will allow the operation to use any number of `shared_buffers`. Otherwise valid sizes range from `128 kB` to `16 GB`. If the specified size would exceed 1/8 the size of `shared_buffers`, the size is silently capped to that value. The default value is `2MB`. If this value is specified without units, it is taken as kilobytes. This parameter can be set at any time. It can be overridden for [VACUUM](https://www.postgresql.org/docs/19/sql-vacuum.html) and [ANALYZE](https://www.postgresql.org/docs/19/sql-analyze.html) when passing the `BUFFER_USAGE_LIMIT` option. Higher settings can allow `VACUUM` and `ANALYZE` to run more quickly, but having too large a setting may cause too many other useful pages to be evicted from shared buffers.
