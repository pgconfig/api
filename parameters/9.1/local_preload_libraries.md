---
name: "local_preload_libraries"
version: "9.1"
type: "string"
category: "Client Connection Defaults / Other Defaults"
short_desc: "Lists shared libraries to preload into each backend."
context: "backend"
default: ""
url: "https://www.postgresql.org/docs/9.1/runtime-config-client.html#GUC-LOCAL-PRELOAD-LIBRARIES"
---

This variable specifies one or more shared libraries that are to be preloaded at connection start. If more than one library is to be loaded, separate their names with commas. All library names are converted to lower case unless double-quoted. This parameter cannot be changed after the start of a particular session.

Because this is not a superuser-only option, the libraries that can be loaded are restricted to those appearing in the `plugins` subdirectory of the installation's standard library directory. (It is the database administrator's responsibility to ensure that only "safe" libraries are installed there.) Entries in `local_preload_libraries` can specify this directory explicitly, for example `$libdir/plugins/mylib`, or just specify the library name — `mylib` would have the same effect as `$libdir/plugins/mylib`.

Unlike [`shared_preload_libraries`](https://www.postgresql.org/docs/9.1/runtime-config-resource.html#GUC-SHARED-PRELOAD-LIBRARIES), there is no performance advantage to loading a library at session start rather than when it is first used. Rather, the intent of this feature is to allow debugging or performance-measurement libraries to be loaded into specific sessions without an explicit `LOAD` command being given. For example, debugging could be enabled for a session by setting this parameter via the `PGOPTIONS` environment variable.

If a specified library is not found, the connection attempt will fail.

Every PostgreSQL-supported library has a "magic block" that is checked to guarantee compatibility. For this reason, non-PostgreSQL libraries cannot be loaded in this way.
