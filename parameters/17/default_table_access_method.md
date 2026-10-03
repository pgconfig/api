---
name: "default_table_access_method"
version: "17"
type: "string"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the default table access method for new tables."
context: "user"
default: "heap"
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-DEFAULT-TABLE-ACCESS-METHOD"
---

This parameter specifies the default table access method to use when creating tables or materialized views if the `CREATE` command does not explicitly specify an access method, or when `SELECT ... INTO` is used, which does not allow specifying a table access method. The default is `heap`.
