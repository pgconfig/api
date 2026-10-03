---
name: "temp_tablespaces"
version: "19"
type: "string"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the tablespace(s) to use for temporary tables and sort files."
extra_desc: "An empty string means use the database's default tablespace."
context: "user"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-TEMP-TABLESPACES"
---

This variable specifies tablespaces in which to create temporary objects (temp tables and indexes on temp tables) when a `CREATE` command does not explicitly specify a tablespace. Temporary files for purposes such as sorting large data sets are also created in these tablespaces.

The value is a list of names of tablespaces. When there is more than one name in the list, PostgreSQL chooses a random member of the list each time a temporary object is to be created; except that within a transaction, successively created temporary objects are placed in successive tablespaces from the list. If the selected element of the list is an empty string, PostgreSQL will automatically use the default tablespace of the current database instead.

When `temp_tablespaces` is set interactively, specifying a nonexistent tablespace is an error, as is specifying a tablespace for which the user does not have `CREATE` privilege. However, when using a previously set value, nonexistent tablespaces are ignored, as are tablespaces for which the user lacks `CREATE` privilege. In particular, this rule applies when using a value set in `postgresql.conf`.

The default value is an empty string, which results in all temporary objects being created in the default tablespace of the current database.

See also [`default_tablespace`](https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-DEFAULT-TABLESPACE).
