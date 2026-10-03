---
name: "autovacuum_max_workers"
version: "16"
type: "integer"
category: "Autovacuum"
short_desc: "Sets the maximum number of simultaneously running autovacuum worker processes."
context: "postmaster"
default: "3"
min: "1"
max: "262143"
url: "https://www.postgresql.org/docs/16/runtime-config-autovacuum.html#GUC-AUTOVACUUM-MAX-WORKERS"
---

Specifies the maximum number of autovacuum processes (other than the autovacuum launcher) that may be running at any one time. The default is three. This parameter can only be set at server start.
