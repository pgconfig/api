---
name: "shared_memory_type"
version: "13"
type: "enum"
category: "Resource Usage / Memory"
short_desc: "Selects the shared memory implementation used for the main shared memory region."
context: "postmaster"
default: "mmap"
values: ["sysv", "mmap"]
url: "https://www.postgresql.org/docs/13/runtime-config-resource.html#GUC-SHARED-MEMORY-TYPE"
---

Specifies the shared memory implementation that the server should use for the main shared memory region that holds PostgreSQL's shared buffers and other shared data. Possible values are `mmap` (for anonymous shared memory allocated using `mmap`), `sysv` (for System V shared memory allocated via `shmget`) and `windows` (for Windows shared memory). Not all values are supported on all platforms; the first supported option is the default for that platform. The use of the `sysv` option, which is not the default on any platform, is generally discouraged because it typically requires non-default kernel settings to allow for large allocations (see [Shared Memory and Semaphores](https://www.postgresql.org/docs/13/kernel-resources.html#SYSVIPC)). This parameter can only be set at server start.
