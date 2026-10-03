---
name: "vacuum_cleanup_index_scale_factor"
version: "12"
type: "floating point"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Number of tuple inserts prior to index cleanup as a fraction of reltuples."
context: "user"
default: "0.1"
min: "0"
max: "1e+10"
url: "https://www.postgresql.org/docs/12/runtime-config-client.html#GUC-VACUUM-CLEANUP-INDEX-SCALE-FACTOR"
---

Specifies the fraction of the total number of heap tuples counted in the previous statistics collection that can be inserted without incurring an index scan at the `VACUUM` cleanup stage. This setting currently applies to B-tree indexes only.

If no tuples were deleted from the heap, B-tree indexes are still scanned at the `VACUUM` cleanup stage when at least one of the following conditions is met: the index statistics are stale, or the index contains deleted pages that can be recycled during cleanup. Index statistics are considered to be stale if the number of newly inserted tuples exceeds the `vacuum_cleanup_index_scale_factor` fraction of the total number of heap tuples detected by the previous statistics collection. The total number of heap tuples is stored in the index meta-page. Note that the meta-page does not include this data until `VACUUM` finds no dead tuples, so B-tree index scan at the cleanup stage can only be skipped if the second and subsequent `VACUUM` cycles detect no dead tuples.

The value can range from `0` to `10000000000`. When `vacuum_cleanup_index_scale_factor` is set to `0`, index scans are never skipped during `VACUUM` cleanup. The default value is `0.1`.
