---
name: "local_preload_libraries"
version: "10"
type: "string"
category: "Client Connection Defaults / Shared Library Preloading"
short_desc: "Lists unprivileged shared libraries to preload into each backend."
context: "user"
default: ""
url: "https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-LOCAL-PRELOAD-LIBRARIES"
---

This variable specifies one or more shared libraries that are to be preloaded at connection start. It contains a comma-separated list of library names, where each name is interpreted as for the [LOAD](https://www.postgresql.org/docs/10/sql-load.html) command. Whitespace between entries is ignored; surround a library name with double quotes if you need to include whitespace or commas in the name. The parameter value only takes effect at the start of the connection. Subsequent changes have no effect. If a specified library is not found, the connection attempt will fail.

This option can be set by any user. Because of that, the libraries that can be loaded are restricted to those appearing in the `plugins` subdirectory of the installation's standard library directory. (It is the database administrator's responsibility to ensure that only "safe" libraries are installed there.) Entries in `local_preload_libraries` can specify this directory explicitly, for example `$libdir/plugins/mylib`, or just specify the library name — `mylib` would have the same effect as `$libdir/plugins/mylib`.

The intent of this feature is to allow unprivileged users to load debugging or performance-measurement libraries into specific sessions without requiring an explicit `LOAD` command. To that end, it would be typical to set this parameter using the `PGOPTIONS` environment variable on the client or by using `ALTER ROLE SET`.

However, unless a module is specifically designed to be used in this way by non-superusers, this is usually not the right setting to use. Look at [`session_preload_libraries`](https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-SESSION-PRELOAD-LIBRARIES) instead.
