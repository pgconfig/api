---
name: "synchronous_commit"
version: "9.1"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the current transaction's synchronization level."
context: "user"
default: "on"
values: ["local", "on", "off"]
url: "https://www.postgresql.org/docs/9.1/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT"
---

Specifies whether transaction commit will wait for WAL records to be written to disk before the command returns a "success" indication to the client. Valid values are `on`, `local`, and `off`. The default, and safe, value is `on`. When `off`, there can be a delay between when success is reported to the client and when the transaction is really guaranteed to be safe against a server crash. (The maximum delay is three times [`wal_writer_delay`](https://www.postgresql.org/docs/9.1/runtime-config-wal.html#GUC-WAL-WRITER-DELAY).) Unlike [`fsync`](https://www.postgresql.org/docs/9.1/runtime-config-wal.html#GUC-FSYNC), setting this parameter to `off` does not create any risk of database inconsistency: an operating system or database crash might result in some recent allegedly-committed transactions being lost, but the database state will be just the same as if those transactions had been aborted cleanly. So, turning `synchronous_commit` off can be a useful alternative when performance is more important than exact certainty about the durability of a transaction. For more discussion see [Asynchronous Commit](https://www.postgresql.org/docs/9.1/wal-async-commit.html).

If [`synchronous_standby_names`](https://www.postgresql.org/docs/9.1/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES) is set, this parameter also controls whether or not transaction commit will wait for the transaction's WAL records to be flushed to disk and replicated to the standby server. The commit wait will last until a reply from the current synchronous standby indicates it has written the commit record of the transaction to durable storage. If synchronous replication is in use, it will normally be sensible either to wait both for WAL records to reach both the local and remote disks, or to allow the transaction to commit asynchronously. However, the special value `local` is available for transactions that wish to wait for local flush to disk, but not synchronous replication.

This parameter can be changed at any time; the behavior for any one transaction is determined by the setting in effect when it commits. It is therefore possible, and useful, to have some transactions commit synchronously and others asynchronously. For example, to make a single multistatement transaction commit asynchronously when the default is the opposite, issue `SET LOCAL synchronous_commit TO OFF` within the transaction.
