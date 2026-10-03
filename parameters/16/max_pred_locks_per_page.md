---
name: "max_pred_locks_per_page"
version: "16"
type: "integer"
category: "Lock Management"
short_desc: "Sets the maximum number of predicate-locked tuples per page."
extra_desc: "If more than this number of tuples on the same page are locked by a connection, those locks are replaced by a page-level lock."
context: "sighup"
default: "2"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-locks.html#GUC-MAX-PRED-LOCKS-PER-PAGE"
---

This controls how many rows on a single page can be predicate-locked before the lock is promoted to covering the whole page. The default is 2. This parameter can only be set in the `postgresql.conf` file or on the server command line.
