---
name: "autovacuum_naptime"
version: "19"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Time to sleep between autovacuum runs."
context: "sighup"
unit: "s"
default: "60"
min: "1"
max: "2147483"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-NAPTIME"
---

Specifies the minimum delay between autovacuum runs on any given database. In each round the daemon examines the database and issues `VACUUM` and `ANALYZE` commands as needed for tables in that database. If this value is specified without units, it is taken as seconds. The default is one minute (`1min`). This parameter can only be set in the `postgresql.conf` file or on the server command line.
