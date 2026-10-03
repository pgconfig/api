---
name: "commit_delay"
version: "9.1"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the delay in microseconds between transaction commit and flushing WAL to disk."
context: "user"
default: "0"
min: "0"
max: "100000"
url: "https://www.postgresql.org/docs/9.1/runtime-config-wal.html#GUC-COMMIT-DELAY"
---

When the commit data for a transaction is flushed to disk, any additional commits ready at that time are also flushed out. `commit_delay` adds a time delay, set in microseconds, before a transaction attempts to flush the WAL buffer out to disk. A nonzero delay can allow more transactions to be committed with only one flush operation, if system load is high enough that additional transactions become ready to commit within the given interval. But the delay is just wasted if no other transactions become ready to commit. Therefore, the delay is only performed if at least `commit_siblings` other transactions are active at the instant that a server process has written its commit record. The default `commit_delay` is zero (no delay). Since all pending commit data will be written at every flush regardless of this setting, it is rare that adding delay by increasing this parameter will actually improve performance.
