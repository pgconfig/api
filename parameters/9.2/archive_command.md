---
name: "archive_command"
version: "9.2"
type: "string"
category: "Write-Ahead Log / Archiving"
short_desc: "Sets the shell command that will be called to archive a WAL file."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/9.2/runtime-config-wal.html#GUC-ARCHIVE-COMMAND"
---

The shell command to execute to archive a completed WAL file segment. Any `%p` in the string is replaced by the path name of the file to archive, and any `%f` is replaced by only the file name. (The path name is relative to the working directory of the server, i.e., the cluster's data directory.) Use `%%` to embed an actual `%` character in the command. It is important for the command to return a zero exit status only if it succeeds. For more information see [Setting Up WAL Archiving](https://www.postgresql.org/docs/9.2/continuous-archiving.html#BACKUP-ARCHIVING-WAL).

This parameter can only be set in the `postgresql.conf` file or on the server command line. It is ignored unless `archive_mode` was enabled at server start. If `archive_command` is an empty string (the default) while `archive_mode` is enabled, WAL archiving is temporarily disabled, but the server continues to accumulate WAL segment files in the expectation that a command will soon be provided. Setting `archive_command` to a command that does nothing but return true, e.g. `/bin/true` (`REM` on Windows), effectively disables archiving, but also breaks the chain of WAL files needed for archive recovery, so it should only be used in unusual circumstances.
