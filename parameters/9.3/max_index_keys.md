---
name: "max_index_keys"
version: "9.3"
type: "integer"
category: "Preset Options"
short_desc: "Shows the maximum number of index keys."
context: "internal"
default: "32"
min: "32"
max: "32"
url: "https://www.postgresql.org/docs/9.3/runtime-config-preset.html#GUC-MAX-INDEX-KEYS"
---

Reports the maximum number of index keys. It is determined by the value of `INDEX_MAX_KEYS` when building the server. The default value is 32 keys.
