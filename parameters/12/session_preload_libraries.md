---
name: "session_preload_libraries"
version: "12"
type: "string"
category: "Client Connection Defaults / Shared Library Preloading"
short_desc: "Lists shared libraries to preload into each backend."
context: "superuser"
default: ""
url: "https://www.postgresql.org/docs/12/runtime-config-client.html#GUC-SESSION-PRELOAD-LIBRARIES"
---

This variable specifies one or more shared libraries that are to be preloaded at connection start. It contains a comma-separated list of library names, where each name is interpreted as for the [LOAD](https://www.postgresql.org/docs/12/sql-load.html) command. Whitespace between entries is ignored; surround a library name with double quotes if you need to include whitespace or commas in the name. The parameter value only takes effect at the start of the connection. Subsequent changes have no effect. If a specified library is not found, the connection attempt will fail. Only superusers can change this setting.

The intent of this feature is to allow debugging or performance-measurement libraries to be loaded into specific sessions without an explicit `LOAD` command being given. For example, [auto_explain](https://www.postgresql.org/docs/12/auto-explain.html) could be enabled for all sessions under a given user name by setting this parameter with `ALTER ROLE SET`. Also, this parameter can be changed without restarting the server (but changes only take effect when a new session is started), so it is easier to add new modules this way, even if they should apply to all sessions.

Unlike [`shared_preload_libraries`](https://www.postgresql.org/docs/12/runtime-config-client.html#GUC-SHARED-PRELOAD-LIBRARIES), there is no large performance advantage to loading a library at session start rather than when it is first used. There is some advantage, however, when connection pooling is used.
