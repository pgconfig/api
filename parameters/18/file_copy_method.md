---
name: "file_copy_method"
version: "18"
type: "enum"
category: "Resource Usage / Disk"
short_desc: "Selects the file copy method."
context: "user"
default: "copy"
values: ["copy", "clone"]
url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-FILE-COPY-METHOD"
---

Specifies the method used to copy files. Possible values are `COPY` (default) and `CLONE` (if operating support is available).

This parameter affects:

- `CREATE DATABASE ... STRATEGY=FILE_COPY`
- `ALTER DATABASE ... SET TABLESPACE ...`

`CLONE` uses the `copy_file_range()` (Linux, FreeBSD) or `copyfile` (macOS) system calls, giving the kernel the opportunity to share disk blocks or push work down to lower layers on some file systems.
