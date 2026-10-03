---
name: "default_tablespace"
version: "16"
type: "string"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the default tablespace to create tables and indexes in."
extra_desc: "An empty string selects the database's default tablespace."
context: "user"
default: ""
url: "https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-DEFAULT-TABLESPACE"
---

This variable specifies the default tablespace in which to create objects (tables and indexes) when a `CREATE` command does not explicitly specify a tablespace.

The value is either the name of a tablespace, or an empty string to specify using the default tablespace of the current database. If the value does not match the name of any existing tablespace, PostgreSQL will automatically use the default tablespace of the current database. If a nondefault tablespace is specified, the user must have `CREATE` privilege for it, or creation attempts will fail.

This variable is not used for temporary tables; for them, [`temp_tablespaces`](https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-TEMP-TABLESPACES) is consulted instead.

This variable is also not used when creating databases. By default, a new database inherits its tablespace setting from the template database it is copied from.

If this parameter is set to a value other than the empty string when a partitioned table is created, the partitioned table's tablespace will be set to that value, which will be used as the default tablespace for partitions created in the future, even if `default_tablespace` has changed since then.

For more information on tablespaces, see [Tablespaces](https://www.postgresql.org/docs/16/manage-ag-tablespaces.html).
