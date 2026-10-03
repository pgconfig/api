---
name: "file_extend_method"
version: "19"
type: "enum"
category: "Resource Usage / Disk"
short_desc: "Selects the method used for extending data files."
context: "sighup"
default: "posix_fallocate"
values: ["posix_fallocate", "write_zeros"]
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-FILE-EXTEND-METHOD"
---

Specifies the method used to extend data files during bulk operations such as `COPY`. The first available option is used as the default, depending on the operating system:

- `posix_fallocate` (Unix) uses the standard POSIX interface for allocating disk space, but is missing on some systems. If it is present but the underlying file system doesn't support it, this option silently falls back to `write_zeros`. Current versions of BTRFS are known to disable compression when this option is used. This is the default on systems that have the function.
- `write_zeros` extends files by writing out blocks of zero bytes. This is the default on systems that don't have the function `posix_fallocate`.

The `write_zeros` method is always used when data files are extended by 8 blocks or fewer.
