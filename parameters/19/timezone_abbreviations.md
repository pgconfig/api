---
name: "timezone_abbreviations"
version: "19"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Selects a file of time zone abbreviations."
context: "user"
url: "https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-TIMEZONE-ABBREVIATIONS"
---

Sets the collection of additional time zone abbreviations that will be accepted by the server for datetime input (beyond any abbreviations defined by the current `TimeZone` setting). The default is `'Default'`, which is a collection that works in most of the world; there are also `'Australia'` and `'India'`, and other collections can be defined for a particular installation. See [Date/Time Configuration Files](https://www.postgresql.org/docs/19/datetime-config-files.html) for more information.
