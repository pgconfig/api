---
name: "archive_library"
version: "16"
type: "string"
category: "Write-Ahead Log / Archiving"
short_desc: "Sets the library that will be called to archive a WAL file."
extra_desc: "An empty string indicates that \"archive_command\" should be used."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-ARCHIVE-LIBRARY"
---

The library to use for archiving completed WAL file segments. If set to an empty string (the default), archiving via shell is enabled, and [`archive_command`](https://www.postgresql.org/docs/16/runtime-config-wal.html#GUC-ARCHIVE-COMMAND) is used. If both `archive_command` and `archive_library` are set, an error will be raised. Otherwise, the specified shared library is used for archiving. The WAL archiver process is restarted by the postmaster when this parameter changes. For more information, see [Setting Up WAL Archiving](https://www.postgresql.org/docs/16/continuous-archiving.html#BACKUP-ARCHIVING-WAL) and [Archive Modules](https://www.postgresql.org/docs/16/archive-modules.html).

This parameter can only be set in the `postgresql.conf` file or on the server command line.
