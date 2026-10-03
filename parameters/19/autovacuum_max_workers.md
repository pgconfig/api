---
name: "autovacuum_max_workers"
version: "19"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Sets the maximum number of simultaneously running autovacuum worker processes."
context: "sighup"
default: "3"
min: "1"
max: "262143"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-MAX-WORKERS"
---

Specifies the maximum number of autovacuum processes (other than the autovacuum launcher) that may be running at any one time. The default is `3`. This parameter can only be set in the `postgresql.conf` file or on the server command line.

Note that a setting for this value which is higher than [`autovacuum_worker_slots`](https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-WORKER-SLOTS) will have no effect, since autovacuum workers are taken from the pool of slots established by that setting.
