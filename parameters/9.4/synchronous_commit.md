---
name: "synchronous_commit"
version: "9.4"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the current transaction's synchronization level."
context: "user"
default: "on"
values: ["local", "remote_write", "on", "off"]
url: "https://www.postgresql.org/docs/9.4/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT"
---

Specifies whether transaction commit will wait for WAL records to be written to disk before the command returns a "success" indication to the client. Valid values are `on`, `remote_write`, `local`, and `off`. The default, and safe, setting is `on`. When `off`, there can be a delay between when success is reported to the client and when the transaction is really guaranteed to be safe against a server crash. (The maximum delay is three times [`wal_writer_delay`](https://www.postgresql.org/docs/9.4/runtime-config-wal.html#GUC-WAL-WRITER-DELAY).) Unlike [`fsync`](https://www.postgresql.org/docs/9.4/runtime-config-wal.html#GUC-FSYNC), setting this parameter to `off` does not create any risk of database inconsistency: an operating system or database crash might result in some recent allegedly-committed transactions being lost, but the database state will be just the same as if those transactions had been aborted cleanly. So, turning `synchronous_commit` off can be a useful alternative when performance is more important than exact certainty about the durability of a transaction. For more discussion see [Asynchronous Commit](https://www.postgresql.org/docs/9.4/wal-async-commit.html).

If [`synchronous_standby_names`](https://www.postgresql.org/docs/9.4/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES) is set, this parameter also controls whether or not transaction commits will wait for the transaction's WAL records to be replicated to the standby server. When set to `on`, commits will wait until a reply from the current synchronous standby indicates it has received the commit record of the transaction and flushed it to disk. This ensures the transaction will not be lost unless both primary and standby suffer corruption of their database storage. When set to `remote_write`, commits will wait until a reply from the current synchronous standby indicates it has received the commit record of the transaction and written it out to the standby's operating system, but the data has not necessarily reached stable storage on the standby. This setting is sufficient to ensure data preservation even if the standby instance of PostgreSQL were to crash, but not if the standby suffers an operating-system-level crash.

When synchronous replication is in use, it will normally be sensible either to wait for both local flush to disk and replication of WAL records, or to allow the transaction to commit asynchronously. However, the setting `local` is available for transactions that wish to wait for local flush to disk, but not synchronous replication. If `synchronous_standby_names` is not set, the settings `on`, `remote_write` and `local` all provide the same synchronization level: transaction commits only wait for local flush to disk.

This parameter can be changed at any time; the behavior for any one transaction is determined by the setting in effect when it commits. It is therefore possible, and useful, to have some transactions commit synchronously and others asynchronously. For example, to make a single multistatement transaction commit asynchronously when the default is the opposite, issue `SET LOCAL synchronous_commit TO OFF` within the transaction.
