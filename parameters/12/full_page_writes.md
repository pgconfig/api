---
name: "full_page_writes"
version: "12"
type: "boolean"
category: "Write-Ahead Log / Settings"
short_desc: "Writes full pages to WAL when first modified after a checkpoint."
extra_desc: "A page write in process during an operating system crash might be only partially written to disk.  During recovery, the row changes stored in WAL are not enough to recover.  This option writes pages when first modified after a checkpoint to WAL so full recovery is possible."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-wal.html#GUC-FULL-PAGE-WRITES"
---

When this parameter is on, the PostgreSQL server writes the entire content of each disk page to WAL during the first modification of that page after a checkpoint. This is needed because a page write that is in process during an operating system crash might be only partially completed, leading to an on-disk page that contains a mix of old and new data. The row-level change data normally stored in WAL will not be enough to completely restore such a page during post-crash recovery. Storing the full page image guarantees that the page can be correctly restored, but at the price of increasing the amount of data that must be written to WAL. (Because WAL replay always starts from a checkpoint, it is sufficient to do this during the first change of each page after a checkpoint. Therefore, one way to reduce the cost of full-page writes is to increase the checkpoint interval parameters.)

Turning this parameter off speeds normal operation, but might lead to either unrecoverable data corruption, or silent data corruption, after a system failure. The risks are similar to turning off `fsync`, though smaller, and it should be turned off only based on the same circumstances recommended for that parameter.

Turning off this parameter does not affect use of WAL archiving for point-in-time recovery (PITR) (see [Continuous Archiving and Point-in-Time Recovery (PITR)](https://www.postgresql.org/docs/12/continuous-archiving.html)).

This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `on`.
