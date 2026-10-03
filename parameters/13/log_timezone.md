---
name: "log_timezone"
version: "13"
type: "string"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the time zone to use in log messages."
context: "sighup"
default: "GMT"
url: "https://www.postgresql.org/docs/13/runtime-config-logging.html#GUC-LOG-TIMEZONE"
---

Sets the time zone used for timestamps written in the server log. Unlike [`TimeZone`](https://www.postgresql.org/docs/13/runtime-config-client.html#GUC-TIMEZONE), this value is cluster-wide, so that all sessions will report timestamps consistently. The built-in default is `GMT`, but that is typically overridden in `postgresql.conf`; initdb will install a setting there corresponding to its system environment. See [Time Zones](https://www.postgresql.org/docs/13/datatype-datetime.html#DATATYPE-TIMEZONES) for more information. This parameter can only be set in the `postgresql.conf` file or on the server command line.
