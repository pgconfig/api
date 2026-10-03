---
name: "max_notify_queue_pages"
version: "19"
type: "integer"
category: "Resource Usage / Disk"
short_desc: "Sets the maximum number of allocated pages for NOTIFY / LISTEN queue."
context: "postmaster"
default: "1048576"
min: "64"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-NOTIFY-QUEUE-PAGES"
---

Specifies the maximum amount of allocated pages for [NOTIFY](https://www.postgresql.org/docs/19/sql-notify.html) / [LISTEN](https://www.postgresql.org/docs/19/sql-listen.html) queue. The default value is 1048576. For 8 KB pages it allows to consume up to 8 GB of disk space. This parameter can only be set at server start.
