---
name: "commit_delay"
version: "9.4"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the delay in microseconds between transaction commit and flushing WAL to disk."
context: "superuser"
default: "0"
min: "0"
max: "100000"
url: "https://www.postgresql.org/docs/9.4/runtime-config-wal.html#GUC-COMMIT-DELAY"
---

`commit_delay` adds a time delay, measured in microseconds, before a WAL flush is initiated. This can improve group commit throughput by allowing a larger number of transactions to commit via a single WAL flush, if system load is high enough that additional transactions become ready to commit within the given interval. However, it also increases latency by up to `commit_delay` microseconds for each WAL flush. Because the delay is just wasted if no other transactions become ready to commit, a delay is only performed if at least `commit_siblings` other transactions are active when a flush is about to be initiated. Also, no delays are performed if `fsync` is disabled. The default `commit_delay` is zero (no delay). Only superusers can change this setting.

In PostgreSQL releases prior to 9.3, `commit_delay` behaved differently and was much less effective: it affected only commits, rather than all WAL flushes, and waited for the entire configured delay even if the WAL flush was completed sooner. Beginning in PostgreSQL 9.3, the first process that becomes ready to flush waits for the configured interval, while subsequent processes wait only until the leader completes the flush operation.
