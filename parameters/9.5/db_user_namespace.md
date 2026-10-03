---
name: "db_user_namespace"
version: "9.5"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Enables per-database user names."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/9.5/runtime-config-connection.html#GUC-DB-USER-NAMESPACE"
---

This parameter enables per-database user names. It is off by default. This parameter can only be set in the `postgresql.conf` file or on the server command line.

If this is on, you should create users as `username@dbname`. When `username` is passed by a connecting client, `@` and the database name are appended to the user name and that database-specific user name is looked up by the server. Note that when you create users with names containing `@` within the SQL environment, you will need to quote the user name.

With this parameter enabled, you can still create ordinary global users. Simply append `@` when specifying the user name in the client, e.g., `joe@`. The `@` will be stripped off before the user name is looked up by the server.

`db_user_namespace` causes the client's and server's user name representation to differ. Authentication checks are always done with the server's user name so authentication methods must be configured for the server's user name, not the client's. Because `md5` uses the user name as salt on both the client and server, `md5` cannot be used with `db_user_namespace`.

> [!NOTE]
> This feature is intended as a temporary measure until a complete solution is found. At that time, this option will be removed.
