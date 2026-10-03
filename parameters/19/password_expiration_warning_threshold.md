---
name: "password_expiration_warning_threshold"
version: "19"
type: "integer"
category: "Connections and Authentication / Authentication"
short_desc: "Threshold for password expiration warnings."
extra_desc: "0 disables these warnings."
context: "sighup"
unit: "s"
default: "604800"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-PASSWORD-EXPIRATION-WARNING-THRESHOLD"
---

When this parameter is greater than zero, the server will emit a `WARNING` upon successful password authentication if less than this amount of time remains until the authenticated role's password expires. Note that a role's password only expires if a date was specified in a `VALID UNTIL` clause for `CREATE ROLE` or `ALTER ROLE`. If this value is specified without units, it is taken as seconds. The default is 7 days. This parameter can only be set in the `postgresql.conf` file or on the server command line.
