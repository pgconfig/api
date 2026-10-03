---
name: "md5_password_warnings"
version: "19"
type: "boolean"
category: "Connections and Authentication / Authentication"
short_desc: "Enables deprecation warnings for MD5 passwords."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-MD5-PASSWORD-WARNINGS"
---

Controls whether a `WARNING` about MD5 password deprecation is produced upon successful authentication using an MD5-encrypted password or when a `CREATE ROLE` or `ALTER ROLE` statement sets an MD5-encrypted password. The default value is `on`.
