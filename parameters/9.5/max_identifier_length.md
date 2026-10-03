---
name: "max_identifier_length"
version: "9.5"
type: "integer"
category: "Preset Options"
short_desc: "Shows the maximum identifier length."
context: "internal"
default: "63"
min: "63"
max: "63"
url: "https://www.postgresql.org/docs/9.5/runtime-config-preset.html#GUC-MAX-IDENTIFIER-LENGTH"
---

Reports the maximum identifier length. It is determined as one less than the value of `NAMEDATALEN` when building the server. The default value of `NAMEDATALEN` is 64; therefore the default `max_identifier_length` is 63 bytes, which can be less than 63 characters when using multibyte encodings.
