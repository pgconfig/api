---
name: "TimeZone"
version: "9.6"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the time zone for displaying and interpreting time stamps."
context: "user"
default: "GMT"
url: "https://www.postgresql.org/docs/9.6/runtime-config-client.html#GUC-TIMEZONE"
---

Sets the time zone for displaying and interpreting time stamps. The built-in default is `GMT`, but that is typically overridden in `postgresql.conf`; initdb will install a setting there corresponding to its system environment. See [Time Zones](https://www.postgresql.org/docs/9.6/datatype-datetime.html#DATATYPE-TIMEZONES) for more information.
