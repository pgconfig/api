---
name: "local_preload_libraries"
version: "9.4"
type: "string"
category: "Client Connection Defaults / Shared Library Preloading"
short_desc: "Lists unprivileged shared libraries to preload into each backend."
context: "backend"
default: ""
url: "https://www.postgresql.org/docs/9.4/runtime-config-client.html#GUC-LOCAL-PRELOAD-LIBRARIES"
---

This variable specifies one or more shared libraries that are to be preloaded at connection start. This parameter cannot be changed after the start of a particular session. If a specified library is not found, the connection attempt will fail.

This option can be set by any user. Because of that, the libraries that can be loaded are restricted to those appearing in the `plugins` subdirectory of the installation's standard library directory. (It is the database administrator's responsibility to ensure that only "safe" libraries are installed there.) Entries in `local_preload_libraries` can specify this directory explicitly, for example `$libdir/plugins/mylib`, or just specify the library name — `mylib` would have the same effect as `$libdir/plugins/mylib`.

Unless a module is specifically designed to be used in this way by non-superusers, this is usually not the right setting to use. Look at [`session_preload_libraries`](https://www.postgresql.org/docs/9.4/runtime-config-client.html#GUC-SESSION-PRELOAD-LIBRARIES) instead.
