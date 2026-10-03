---
name: "password_encryption"
version: "11"
type: "enum"
category: "Connections and Authentication / Authentication"
short_desc: "Chooses the algorithm for encrypting passwords."
context: "user"
default: "md5"
values: ["md5", "scram-sha-256"]
url: "https://www.postgresql.org/docs/11/runtime-config-connection.html#GUC-PASSWORD-ENCRYPTION"
---

When a password is specified in [CREATE ROLE](https://www.postgresql.org/docs/11/sql-createrole.html) or [ALTER ROLE](https://www.postgresql.org/docs/11/sql-alterrole.html), this parameter determines the algorithm to use to encrypt the password. The default value is `md5`, which stores the password as an MD5 hash (`on` is also accepted, as alias for `md5`). Setting this parameter to `scram-sha-256` will encrypt the password with SCRAM-SHA-256.

Note that older clients might lack support for the SCRAM authentication mechanism, and hence not work with passwords encrypted with SCRAM-SHA-256. See [Password Authentication](https://www.postgresql.org/docs/11/auth-password.html) for more details.
