---
name: "lc_numeric"
version: "13"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the locale for formatting numbers."
context: "user"
default: "C"
url: "https://www.postgresql.org/docs/13/runtime-config-client.html#GUC-LC-NUMERIC"
---

Sets the locale to use for formatting numbers, for example with the `to_char` family of functions. Acceptable values are system-dependent; see [Locale Support](https://www.postgresql.org/docs/13/locale.html) for more information. If this variable is set to the empty string (which is the default) then the value is inherited from the execution environment of the server in a system-dependent way.
