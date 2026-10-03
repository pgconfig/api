---
name: "track_commit_timestamp"
version: "11"
type: "boolean"
category: "Replication"
short_desc: "Collects transaction commit time."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/11/runtime-config-replication.html#GUC-TRACK-COMMIT-TIMESTAMP"
---

Record commit time of transactions. This parameter can only be set in `postgresql.conf` file or on the server command line. The default value is `off`.
