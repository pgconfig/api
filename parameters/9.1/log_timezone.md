---
name: "log_timezone"
version: "9.1"
type: "string"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the time zone to use in log messages."
context: "sighup"
url: "https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-LOG-TIMEZONE"
---

Sets the time zone used for timestamps written in the server log. Unlike [`timezone`](https://www.postgresql.org/docs/9.1/runtime-config-client.html#GUC-TIMEZONE), this value is cluster-wide, so that all sessions will report timestamps consistently. If not explicitly set, the server initializes this variable to the time zone specified by its system environment. See [Time Zones](https://www.postgresql.org/docs/9.1/datatype-datetime.html#DATATYPE-TIMEZONES) for more information. This parameter can only be set in the `postgresql.conf` file or on the server command line.
