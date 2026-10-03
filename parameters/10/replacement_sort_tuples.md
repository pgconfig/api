---
name: "replacement_sort_tuples"
version: "10"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum number of tuples to be sorted using replacement selection."
extra_desc: "When more tuples than this are present, quicksort will be used."
context: "user"
default: "150000"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/10/runtime-config-resource.html#GUC-REPLACEMENT-SORT-TUPLES"
---

When the number of tuples to be sorted is smaller than this number, a sort will produce its first output run using replacement selection rather than quicksort. This may be useful in memory-constrained environments where tuples that are input into larger sort operations have a strong physical-to-logical correlation. Note that this does not include input tuples with an *inverse* correlation. It is possible for the replacement selection algorithm to generate one long run that requires no merging, where use of the default strategy would result in many runs that must be merged to produce a final sorted output. This may allow sort operations to complete sooner.

The default is 150,000 tuples. Note that higher values are typically not much more effective, and may be counter-productive, since the priority queue is sensitive to the size of available CPU cache, whereas the default strategy sorts runs using a *cache oblivious* algorithm. This property allows the default sort strategy to automatically and transparently make effective use of available CPU cache.

Setting `maintenance_work_mem` to its default value usually prevents utility command external sorts (e.g., sorts used by `CREATE INDEX` to build B-Tree indexes) from ever using replacement selection sort, unless the input tuples are quite wide.
