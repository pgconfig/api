---
name: "shared_preload_libraries"
version: "9.2"
type: "string"
category: "Resource Usage / Kernel Resources"
short_desc: "Lists shared libraries to preload into server."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/9.2/runtime-config-resource.html#GUC-SHARED-PRELOAD-LIBRARIES"
---

This variable specifies one or more shared libraries to be preloaded at server start. For example, `'$libdir/mylib'` would cause `mylib.so` (or on some platforms, `mylib.sl`) to be preloaded from the installation's standard library directory. All library names are converted to lower case unless double-quoted. If more than one library is to be loaded, separate their names with commas. This parameter can only be set at server start.

PostgreSQL procedural language libraries can be preloaded in this way, typically by using the syntax `'$libdir/plXXX'` where `XXX` is `pgsql`, `perl`, `tcl`, or `python`.

By preloading a shared library, the library startup time is avoided when the library is first used. However, the time to start each new server process might increase slightly, even if that process never uses the library. So this parameter is recommended only for libraries that will be used in most sessions.

> [!NOTE]
> On Windows hosts, preloading a library at server start will not reduce the time required to start each new server process; each server process will re-load all preload libraries. However, `shared_preload_libraries ` is still useful on Windows hosts because some shared libraries may need to perform certain operations that only take place at postmaster start (for example, a shared library may need to reserve lightweight locks or shared memory and you can't do that after the postmaster has started).

If a specified library is not found, the server will fail to start.

Every PostgreSQL-supported library has a "magic block" that is checked to guarantee compatibility. For this reason, non-PostgreSQL libraries cannot be loaded in this way.
