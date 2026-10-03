---
name: "wal_sync_method"
version: "18"
type: "enum"
category: "Write-Ahead Log / Settings"
short_desc: "Selects the method used for forcing WAL updates to disk."
context: "sighup"
default: "fdatasync"
values: ["fsync", "fdatasync", "open_sync", "open_datasync"]
url: "https://www.postgresql.org/docs/18/runtime-config-wal.html#GUC-WAL-SYNC-METHOD"
---

Method used for forcing WAL updates out to disk. If `fsync` is off then this setting is irrelevant, since WAL file updates will not be forced out at all. Possible values are:

- `open_datasync` (write WAL files with `open()` option `O_DSYNC`)
- `fdatasync` (call `fdatasync()` at each commit)
- `fsync` (call `fsync()` at each commit)
- `fsync_writethrough` (call `fsync()` at each commit, forcing write-through of any disk write cache)
- `open_sync` (write WAL files with `open()` option `O_SYNC`)

Not all of these choices are available on all platforms. The default is the first method in the above list that is supported by the platform, except that `fdatasync` is the default on Linux and FreeBSD. The default is not necessarily ideal; it might be necessary to change this setting or other aspects of your system configuration in order to create a crash-safe configuration or achieve optimal performance. These aspects are discussed in [Reliability](https://www.postgresql.org/docs/18/wal-reliability.html). This parameter can only be set in the `postgresql.conf` file or on the server command line.
