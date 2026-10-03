---
name: "synchronous_commit"
version: "17"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the current transaction's synchronization level."
context: "user"
default: "on"
values: ["local", "remote_write", "remote_apply", "on", "off"]
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT"
---

Specifies how much WAL processing must complete before the database server returns a "success" indication to the client. Valid values are `remote_apply`, `on` (the default), `remote_write`, `local`, and `off`.

If `synchronous_standby_names` is empty, the only meaningful settings are `on` and `off`; `remote_apply`, `remote_write` and `local` all provide the same local synchronization level as `on`. The local behavior of all non-`off` modes is to wait for local flush of WAL to disk. In `off` mode, there is no waiting, so there can be a delay between when success is reported to the client and when the transaction is later guaranteed to be safe against a server crash. (The maximum delay is three times [`wal_writer_delay`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-WAL-WRITER-DELAY).) Unlike [`fsync`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-FSYNC), setting this parameter to `off` does not create any risk of database inconsistency: an operating system or database crash might result in some recent allegedly-committed transactions being lost, but the database state will be just the same as if those transactions had been aborted cleanly. So, turning `synchronous_commit` off can be a useful alternative when performance is more important than exact certainty about the durability of a transaction. For more discussion see [Asynchronous Commit](https://www.postgresql.org/docs/17/wal-async-commit.html).

If [`synchronous_standby_names`](https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES) is non-empty, `synchronous_commit` also controls whether transaction commits will wait for their WAL records to be processed on the standby server(s).

When set to `remote_apply`, commits will wait until replies from the current synchronous standby(s) indicate they have received the commit record of the transaction and applied it, so that it has become visible to queries on the standby(s), and also written to durable storage on the standbys. This will cause much larger commit delays than previous settings since it waits for WAL replay. When set to `on`, commits wait until replies from the current synchronous standby(s) indicate they have received the commit record of the transaction and flushed it to durable storage. This ensures the transaction will not be lost unless both the primary and all synchronous standbys suffer corruption of their database storage. When set to `remote_write`, commits will wait until replies from the current synchronous standby(s) indicate they have received the commit record of the transaction and written it to their file systems. This setting ensures data preservation if a standby instance of PostgreSQL crashes, but not if the standby suffers an operating-system-level crash because the data has not necessarily reached durable storage on the standby. The setting `local` causes commits to wait for local flush to disk, but not for replication. This is usually not desirable when synchronous replication is in use, but is provided for completeness.

This parameter can be changed at any time; the behavior for any one transaction is determined by the setting in effect when it commits. It is therefore possible, and useful, to have some transactions commit synchronously and others asynchronously. For example, to make a single multistatement transaction commit asynchronously when the default is the opposite, issue `SET LOCAL synchronous_commit TO OFF` within the transaction.

[synchronous_commit Modes](https://www.postgresql.org/docs/17/runtime-config-wal.html#SYNCHRONOUS-COMMIT-MATRIX) summarizes the capabilities of the `synchronous_commit` settings.

**synchronous_commit Modes**

| synchronous_commit setting | local durable commit | standby durable commit after PG crash | standby durable commit after OS crash | standby query consistency |
| --- | --- | --- | --- | --- |
| remote_apply | • | • | • | • |
| on | • | • | • |  |
| remote_write | • | • |  |  |
| local | • |  |  |  |
| off |  |  |  |  |
