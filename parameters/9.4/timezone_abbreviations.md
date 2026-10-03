---
name: "timezone_abbreviations"
version: "9.4"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Selects a file of time zone abbreviations."
context: "user"
url: "https://www.postgresql.org/docs/9.4/runtime-config-client.html#GUC-TIMEZONE-ABBREVIATIONS"
---

Sets the collection of time zone abbreviations that will be accepted by the server for datetime input. The default is `'Default'`, which is a collection that works in most of the world; there are also `'Australia'` and `'India'`, and other collections can be defined for a particular installation. See [Date/Time Configuration Files](https://www.postgresql.org/docs/9.4/datetime-config-files.html) for more information.
