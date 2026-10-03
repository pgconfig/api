---
name: "jit_dump_bitcode"
version: "18"
type: "boolean"
category: "Developer Options"
short_desc: "Write out LLVM bitcode to facilitate JIT debugging."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-JIT-DUMP-BITCODE"
---

Writes the generated LLVM IR out to the file system, inside [`data_directory`](https://www.postgresql.org/docs/18/runtime-config-file-locations.html#GUC-DATA-DIRECTORY). This is only useful for working on the internals of the JIT implementation. The default setting is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.
