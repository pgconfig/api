---
name: "recovery_end_command"
version: "17"
type: "string"
category: "Write-Ahead Log / Archive Recovery"
short_desc: "Sets the shell command that will be executed once at the end of recovery."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-RECOVERY-END-COMMAND"
---

This parameter specifies a shell command that will be executed once only at the end of recovery. This parameter is optional. The purpose of the `recovery_end_command` is to provide a mechanism for cleanup following replication or recovery. Any `%r` is replaced by the name of the file containing the last valid restart point, like in [`archive_cleanup_command`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-ARCHIVE-CLEANUP-COMMAND).

If the command returns a nonzero exit status then a warning log message will be written and the database will proceed to start up anyway. An exception is that if the command was terminated by a signal or an error by the shell (such as command not found), the database will not proceed with startup.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
