---
name: "archive_cleanup_command"
version: "17"
type: "string"
category: "Write-Ahead Log / Archive Recovery"
short_desc: "Sets the shell command that will be executed at every restart point."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-ARCHIVE-CLEANUP-COMMAND"
---

This optional parameter specifies a shell command that will be executed at every restartpoint. The purpose of `archive_cleanup_command` is to provide a mechanism for cleaning up old archived WAL files that are no longer needed by the standby server. Any `%r` is replaced by the name of the file containing the last valid restart point. That is the earliest file that must be *kept* to allow a restore to be restartable, and so all files earlier than `%r` may be safely removed. This information can be used to truncate the archive to just the minimum required to support restart from the current restore. The [pg_archivecleanup](https://www.postgresql.org/docs/17/pgarchivecleanup.html) module is often used in `archive_cleanup_command` for single-standby configurations, for example:

```
archive_cleanup_command = 'pg_archivecleanup /mnt/server/archivedir %r'
```

Note however that if multiple standby servers are restoring from the same archive directory, you will need to ensure that you do not delete WAL files until they are no longer needed by any of the servers. `archive_cleanup_command` would typically be used in a warm-standby configuration (see [Log-Shipping Standby Servers](https://www.postgresql.org/docs/17/warm-standby.html)). Write `%%` to embed an actual `%` character in the command.

If the command returns a nonzero exit status then a warning log message will be written. An exception is that if the command was terminated by a signal or an error by the shell (such as command not found), a fatal error will be raised.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
