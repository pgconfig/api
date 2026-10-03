---
name: "debug_io_direct"
version: "19"
type: "string"
category: "Developer Options"
short_desc: "Use direct I/O for file access."
extra_desc: "An empty string disables direct I/O."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-DEBUG-IO-DIRECT"
---

Ask the kernel to minimize caching effects for relation data and WAL files using `O_DIRECT` (most Unix-like systems), `F_NOCACHE` (macOS) or `FILE_FLAG_NO_BUFFERING` (Windows).

May be set to an empty string (the default) to disable use of direct I/O, or a comma-separated list of operations that should use direct I/O. The valid options are `data` for main data files, `wal` for WAL files, and `wal_init` for WAL files when being initially allocated. This parameter can only be set at server start.

Some operating systems and file systems do not support direct I/O, so non-default settings may be rejected at startup or cause errors.

Currently this feature reduces performance, and is intended for developer testing only.
