---
name: "commit_siblings"
version: "16"
type: "integer"
category: "Write-Ahead Log / Settings"
short_desc: "Sets the minimum number of concurrent open transactions required before performing commit_delay."
context: "user"
default: "5"
min: "0"
max: "1000"
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-COMMIT-SIBLINGS"
---

Minimum number of concurrent open transactions to require before performing the `commit_delay` delay. A larger value makes it more probable that at least one other transaction will become ready to commit during the delay interval. The default is five transactions.
