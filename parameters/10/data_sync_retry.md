---
name: "data_sync_retry"
version: "10"
type: "boolean"
category: "Error Handling"
short_desc: "Whether to continue running after a failure to sync data files."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/10/runtime-config-error-handling.html#GUC-DATA-SYNC-RETRY"
---

When set to false, which is the default, PostgreSQL will raise a PANIC-level error on failure to flush modified data files to the filesystem. This causes the database server to crash. This parameter can only be set at server start.

On some operating systems, the status of data in the kernel's page cache is unknown after a write-back failure. In some cases it might have been entirely forgotten, making it unsafe to retry; the second attempt may be reported as successful, when in fact the data has been lost. In these circumstances, the only way to avoid data loss is to recover from the WAL after any failure is reported, preferably after investigating the root cause of the failure and replacing any faulty hardware.

If set to true, PostgreSQL will instead report an error but continue to run so that the data flushing operation can be retried in a later checkpoint. Only set it to true after investigating the operating system's treatment of buffered data in case of write-back failure.
