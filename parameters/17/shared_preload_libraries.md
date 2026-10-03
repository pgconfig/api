---
name: "shared_preload_libraries"
version: "17"
type: "string"
category: "Client Connection Defaults / Shared Library Preloading"
short_desc: "Lists shared libraries to preload into server."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-SHARED-PRELOAD-LIBRARIES"
---

This variable specifies one or more shared libraries to be preloaded at server start. It contains a comma-separated list of library names, where each name is interpreted as for the [`LOAD`](https://www.postgresql.org/docs/17/sql-load.html) command. Whitespace between entries is ignored; surround a library name with double quotes if you need to include whitespace or commas in the name. This parameter can only be set at server start. If a specified library is not found, the server will fail to start.

Some libraries need to perform certain operations that can only take place at postmaster start, such as allocating shared memory, reserving light-weight locks, or starting background workers. Those libraries must be loaded at server start through this parameter. See the documentation of each library for details.

Other libraries can also be preloaded. By preloading a shared library, the library startup time is avoided when the library is first used. However, the time to start each new server process might increase slightly, even if that process never uses the library. So this parameter is recommended only for libraries that will be used in most sessions. Also, changing this parameter requires a server restart, so this is not the right setting to use for short-term debugging tasks, say. Use [`session_preload_libraries`](https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-SESSION-PRELOAD-LIBRARIES) for that instead.

> [!NOTE]
> On Windows hosts, preloading a library at server start will not reduce the time required to start each new server process; each server process will re-load all preload libraries. However, `shared_preload_libraries ` is still useful on Windows hosts for libraries that need to perform operations at postmaster start time.
