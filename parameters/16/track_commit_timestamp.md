---
name: "track_commit_timestamp"
version: "16"
type: "boolean"
category: "Replication / Sending Servers"
short_desc: "Collects transaction commit time."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/16/runtime-config-replication.html#GUC-TRACK-COMMIT-TIMESTAMP"
---

Record commit time of transactions. This parameter can only be set at server start. The default value is `off`.
