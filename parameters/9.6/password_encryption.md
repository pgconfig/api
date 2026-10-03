---
name: "password_encryption"
version: "9.6"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Encrypt passwords."
extra_desc: "When a password is specified in CREATE USER or ALTER USER without writing either ENCRYPTED or UNENCRYPTED, this parameter determines whether the password is to be encrypted."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/9.6/runtime-config-connection.html#GUC-PASSWORD-ENCRYPTION"
---

When a password is specified in [CREATE USER](https://www.postgresql.org/docs/9.6/sql-createuser.html) or [ALTER ROLE](https://www.postgresql.org/docs/9.6/sql-alterrole.html) without writing either `ENCRYPTED` or `UNENCRYPTED`, this parameter determines whether the password is to be encrypted. The default is `on` (encrypt the password).
