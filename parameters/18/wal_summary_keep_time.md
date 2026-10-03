---
name: "wal_summary_keep_time"
version: "18"
type: "integer"
category: "Write-Ahead Log / Summarization"
short_desc: "Time for which WAL summary files should be kept."
extra_desc: "0 disables automatic summary file deletion."
context: "sighup"
unit: "min"
default: "14400"
min: "0"
max: "35791394"
url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-WAL-SUMMARY-KEEP-TIME"
---

Configures the amount of time after which the WAL summarizer automatically removes old WAL summaries. The file timestamp is used to determine which files are old enough to remove. Typically, you should set this comfortably higher than the time that could pass between a backup and a later incremental backup that depends on it. WAL summaries must be available for the entire range of WAL records between the preceding backup and the new one being taken; if not, the incremental backup will fail. If this parameter is set to zero, WAL summaries will not be automatically deleted, but it is safe to manually remove files that you know will not be required for future incremental backups. This parameter can only be set in the `postgresql.conf` file or on the server command line. If this value is specified without units, it is taken as minutes. The default is 10 days. If `summarize_wal = off`, existing WAL summaries will not be removed regardless of the value of this parameter, because the WAL summarizer will not run.
