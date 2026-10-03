---
name: "dynamic_shared_memory_type"
version: "15"
type: "enum"
category: "Resource Usage / Memory"
short_desc: "Selects the dynamic shared memory implementation used."
context: "postmaster"
default: "posix"
values: ["posix", "sysv", "mmap"]
url: "https://www.postgresql.org/docs/15/runtime-config-resource.html#GUC-DYNAMIC-SHARED-MEMORY-TYPE"
---

Specifies the dynamic shared memory implementation that the server should use. Possible values are `posix` (for POSIX shared memory allocated using `shm_open`), `sysv` (for System V shared memory allocated via `shmget`), `windows` (for Windows shared memory), and `mmap` (to simulate shared memory using memory-mapped files stored in the data directory). Not all values are supported on all platforms; the first supported option is usually the default for that platform. The use of the `mmap` option, which is not the default on any platform, is generally discouraged because the operating system may write modified pages back to disk repeatedly, increasing system I/O load; however, it may be useful for debugging, when the `pg_dynshmem` directory is stored on a RAM disk, or when other shared memory facilities are not available. This parameter can only be set at server start.
