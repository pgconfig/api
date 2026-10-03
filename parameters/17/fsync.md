---
name: "fsync"
version: "17"
type: "boolean"
category: "Write-Ahead Log / Settings"
short_desc: "Forces synchronization of updates to disk."
extra_desc: "The server will use the fsync() system call in several places to make sure that updates are physically written to disk. This ensures that a database cluster will recover to a consistent state after an operating system or hardware crash."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-FSYNC"
---

If this parameter is on, the PostgreSQL server will try to make sure that updates are physically written to disk, by issuing `fsync()` system calls or various equivalent methods (see [`wal_sync_method`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-WAL-SYNC-METHOD)). This ensures that the database cluster can recover to a consistent state after an operating system or hardware crash.

While turning off `fsync` is often a performance benefit, this can result in unrecoverable data corruption in the event of a power failure or system crash. Thus it is only advisable to turn off `fsync` if you can easily recreate your entire database from external data.

Examples of safe circumstances for turning off `fsync` include the initial loading of a new database cluster from a backup file, using a database cluster for processing a batch of data after which the database will be thrown away and recreated, or for a read-only database clone which gets recreated frequently and is not used for failover. High quality hardware alone is not a sufficient justification for turning off `fsync`.

For reliable recovery when changing `fsync` off to on, it is necessary to force all modified buffers in the kernel to durable storage. This can be done while the cluster is shutdown or while `fsync` is on by running `initdb --sync-only`, running `sync`, unmounting the file system, or rebooting the server.

In many situations, turning off [`synchronous_commit`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-SYNCHRONOUS-COMMIT) for noncritical transactions can provide much of the potential performance benefit of turning off `fsync`, without the attendant risks of data corruption.

`fsync` can only be set in the `postgresql.conf` file or on the server command line. If you turn this parameter off, also consider turning off [`full_page_writes`](https://www.postgresql.org/docs/17/runtime-config-wal.html#GUC-FULL-PAGE-WRITES).
