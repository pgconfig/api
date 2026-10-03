---
name: "multixact_member_buffers"
version: "17"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the size of the dedicated buffer pool used for the MultiXact member cache."
context: "postmaster"
unit: "8kB"
default: "32"
min: "16"
max: "131072"
url: "https://www.postgresql.org/docs/17/runtime-config-resource.html#GUC-MULTIXACT-MEMBER-BUFFERS"
---

Specifies the amount of shared memory to use to cache the contents of `pg_multixact/members` (see [Contents of PGDATA](https://www.postgresql.org/docs/17/storage-file-layout.html#PGDATA-CONTENTS-TABLE)). If this value is specified without units, it is taken as blocks, that is `BLCKSZ` bytes, typically 8kB. The default value is `32`. This parameter can only be set at server start.
