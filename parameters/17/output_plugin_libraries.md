---
name: "output_plugin_libraries"
version: "17"
type: "string"
category: "Replication / Sending Servers"
short_desc: "Lists libraries that may be named as logical decoding output plugins."
extra_desc: "Users with REPLICATION privileges may only use plugins in this list when creating logical replication slots."
context: "superuser"
default: "pgoutput, test_decoding"
url: "https://www.postgresql.org/docs/17/runtime-config-replication.html#GUC-OUTPUT-PLUGIN-LIBRARIES"
---

Lists the libraries installed in [`dynamic_library_path`](https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-DYNAMIC-LIBRARY-PATH) that are also trusted for use as logical output plugins by replication clients. Any [logical decoding](https://www.postgresql.org/docs/17/logicaldecoding-example.html) or [replication](https://www.postgresql.org/docs/17/protocol-replication.html) requests for other libraries will be refused. All users are subject to this restriction. The default is `'pgoutput, test_decoding'`, which are the two logical output plugins included in the standard PostgreSQL distribution.

The format is a comma-separated list of library names, where each name is interpreted as for the [`LOAD`](https://www.postgresql.org/docs/17/sql-load.html) command (but logical decoding clients must specify a plugin name that *exactly* matches an entry in the list, without variations in case or path structure). Whitespace between entries is ignored; surround a library name with double quotes if you need to include whitespace or commas in the name.

It is the responsibility of the server administrator to ensure that libraries added to this list do not unintentionally give additional privileges to non-superusers when they are loaded into the server.

> [!NOTE]
> When updating the server from a version that does not have the `output_plugin_libraries` parameter, the following query can help construct the list of plugins that are required by all persistent logical replication slots:
>
> ```
> SELECT DISTINCT plugin FROM pg_replication_slots WHERE plugin IS NOT NULL;
> ```
>
> Review the list carefully for safety before adjusting `output_plugin_libraries`.
>
> The above query can only display plugins which were successfully added to replication slots at some point in the past. Newly refused requests will appear in the logs with a message similar to
>
> ```
> ERROR:  library "..." may not be used as an output plugin
> DETAIL:  The configuration parameter "output_plugin_libraries" (currently 'pgoutput, test_decoding') does not name this library as a trusted output plugin.
> HINT:  If it is safe for all REPLICATION users to use this library as an output plugin, add it to "output_plugin_libraries" and reload the server configuration.
> ```
