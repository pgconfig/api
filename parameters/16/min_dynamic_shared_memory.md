---
name: "min_dynamic_shared_memory"
version: "16"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Amount of dynamic shared memory reserved at startup."
context: "postmaster"
unit: "MB"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-MIN-DYNAMIC-SHARED-MEMORY"
---

Specifies the amount of memory that should be allocated at server startup for use by parallel queries. When this memory region is insufficient or exhausted by concurrent queries, new parallel queries try to allocate extra shared memory temporarily from the operating system using the method configured with `dynamic_shared_memory_type`, which may be slower due to memory management overheads. Memory that is allocated at startup with `min_dynamic_shared_memory` is affected by the `huge_pages` setting on operating systems where that is supported, and may be more likely to benefit from larger pages on operating systems where that is managed automatically. The default value is `0` (none). This parameter can only be set at server start.
