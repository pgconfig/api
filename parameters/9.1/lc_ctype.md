---
name: "lc_ctype"
version: "9.1"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Shows the character classification and case conversion locale."
context: "internal"
default: "C"
url: "https://www.postgresql.org/docs/9.1/runtime-config-preset.html#GUC-LC-CTYPE"
---

Reports the locale that determines character classifications. See [Locale Support](https://www.postgresql.org/docs/9.1/locale.html) for more information. This value is determined when a database is created. Ordinarily this will be the same as `lc_collate`, but for special applications it might be set differently.
