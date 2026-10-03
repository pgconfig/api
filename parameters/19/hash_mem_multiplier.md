---
name: "hash_mem_multiplier"
version: "19"
type: "floating point"
category: "Resource Usage / Memory"
short_desc: "Multiple of \"work_mem\" to use for hash tables."
context: "user"
default: "2"
min: "1"
max: "1000"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-HASH-MEM-MULTIPLIER"
---

Used to compute the maximum amount of memory that hash-based operations can use. The final limit is determined by multiplying `work_mem` by `hash_mem_multiplier`. The default value is 2.0, which makes hash-based operations use twice the usual `work_mem` base amount.

Consider increasing `hash_mem_multiplier` in environments where spilling by query operations is a regular occurrence, especially when simply increasing `work_mem` results in memory pressure (memory pressure typically takes the form of intermittent out of memory errors). The default setting of 2.0 is often effective with mixed workloads. Higher settings in the range of 2.0 - 8.0 or more may be effective in environments where `work_mem` has already been increased to 40MB or more.
