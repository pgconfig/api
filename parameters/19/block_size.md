---
name: "block_size"
version: "19"
type: "integer"
category: "Preset Options"
short_desc: "Shows the size of a disk block."
context: "internal"
default: "8192"
min: "8192"
max: "8192"
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-BLOCK-SIZE"
---

Reports the size of a disk block. It is determined by the value of `BLCKSZ` when building the server. The default value is 8192 bytes. The meaning of some configuration variables (such as [`shared_buffers`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-SHARED-BUFFERS)) is influenced by `block_size`. See [Resource Consumption](https://www.postgresql.org/docs/19/runtime-config-resource.html) for information.
