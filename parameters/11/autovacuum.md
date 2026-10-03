---
name: "autovacuum"
version: "11"
type: "boolean"
category: "Autovacuum"
short_desc: "Starts the autovacuum subprocess."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/11/runtime-config-autovacuum.html#GUC-AUTOVACUUM"
---

Controls whether the server should run the autovacuum launcher daemon. This is on by default; however, [`track_counts`](https://www.postgresql.org/docs/11/runtime-config-statistics.html#GUC-TRACK-COUNTS) must also be enabled for autovacuum to work. This parameter can only be set in the `postgresql.conf` file or on the server command line; however, autovacuuming can be disabled for individual tables by changing table storage parameters.

Note that even when this parameter is disabled, the system will launch autovacuum processes if necessary to prevent transaction ID wraparound. See [Preventing Transaction ID Wraparound Failures](https://www.postgresql.org/docs/11/routine-vacuuming.html#VACUUM-FOR-WRAPAROUND) for more information.
