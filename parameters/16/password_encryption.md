---
name: "password_encryption"
version: "16"
type: "enum"
category: "Connections and Authentication / Authentication"
short_desc: "Chooses the algorithm for encrypting passwords."
context: "user"
default: "scram-sha-256"
values: ["md5", "scram-sha-256"]
url: "https://www.postgresql.org/docs/16/runtime-config-connection.html#GUC-PASSWORD-ENCRYPTION"
---

When a password is specified in [CREATE ROLE](https://www.postgresql.org/docs/16/sql-createrole.html) or [ALTER ROLE](https://www.postgresql.org/docs/16/sql-alterrole.html), this parameter determines the algorithm to use to encrypt the password. Possible values are `scram-sha-256`, which will encrypt the password with SCRAM-SHA-256, and `md5`, which stores the password as an MD5 hash. The default is `scram-sha-256`.

Note that older clients might lack support for the SCRAM authentication mechanism, and hence not work with passwords encrypted with SCRAM-SHA-256. See [Password Authentication](https://www.postgresql.org/docs/16/auth-password.html) for more details.
