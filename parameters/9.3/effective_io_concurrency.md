---
name: "effective_io_concurrency"
version: "9.3"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Number of simultaneous requests that can be handled efficiently by the disk subsystem."
extra_desc: "For RAID arrays, this should be approximately the number of drive spindles in the array."
context: "user"
default: "1"
min: "0"
max: "1000"
url: "https://www.postgresql.org/docs/9.3/runtime-config-resource.html#GUC-EFFECTIVE-IO-CONCURRENCY"
---

Sets the number of concurrent disk I/O operations that PostgreSQL expects can be executed simultaneously. Raising this value will increase the number of I/O operations that any individual PostgreSQL session attempts to initiate in parallel. The allowed range is 1 to 1000, or zero to disable issuance of asynchronous I/O requests. Currently, this setting only affects bitmap heap scans.

A good starting point for this setting is the number of separate drives comprising a RAID 0 stripe or RAID 1 mirror being used for the database. (For RAID 5 the parity drive should not be counted.) However, if the database is often busy with multiple queries issued in concurrent sessions, lower values may be sufficient to keep the disk array busy. A value higher than needed to keep the disks busy will only result in extra CPU overhead.

For more exotic systems, such as memory-based storage or a RAID array that is limited by bus bandwidth, the correct value might be the number of I/O paths available. Some experimentation may be needed to find the best value.

Asynchronous I/O depends on an effective `posix_fadvise` function, which some operating systems lack. If the function is not present then setting this parameter to anything but zero will result in an error. On some operating systems (e.g., Solaris), the function is present but does not actually do anything.
