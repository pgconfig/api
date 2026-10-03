---
name: "autovacuum_worker_slots"
version: "19"
type: "integer"
category: "Vacuuming / Automatic Vacuuming"
short_desc: "Sets the number of backend slots to allocate for autovacuum workers."
context: "postmaster"
default: "16"
min: "1"
max: "262143"
url: "https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-WORKER-SLOTS"
---

Specifies the number of backend slots to reserve for autovacuum worker processes. The default is typically 16 slots, but might be less if your kernel settings will not support it (as determined during initdb). This parameter can only be set at server start.

When changing this value, consider also adjusting [`autovacuum_max_workers`](https://www.postgresql.org/docs/19/runtime-config-vacuum.html#GUC-AUTOVACUUM-MAX-WORKERS).
