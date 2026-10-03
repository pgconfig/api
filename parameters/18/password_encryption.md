---
name: "password_encryption"
version: "18"
type: "enum"
category: "Connections and Authentication / Authentication"
short_desc: "Chooses the algorithm for encrypting passwords."
context: "user"
default: "scram-sha-256"
values: ["md5", "scram-sha-256"]
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-PASSWORD-ENCRYPTION"
---

When a password is specified in [CREATE ROLE](https://www.postgresql.org/docs/18/sql-createrole.html) or [ALTER ROLE](https://www.postgresql.org/docs/18/sql-alterrole.html), this parameter determines the algorithm to use to encrypt the password. Possible values are `scram-sha-256`, which will encrypt the password with SCRAM-SHA-256, and `md5`, which stores the password as an MD5 hash. The default is `scram-sha-256`.

Note that older clients might lack support for the SCRAM authentication mechanism, and hence not work with passwords encrypted with SCRAM-SHA-256. See [Password Authentication](https://www.postgresql.org/docs/18/auth-password.html) for more details.

> [!WARNING]
> Support for MD5-encrypted passwords is deprecated and will be removed in a future release of PostgreSQL. Refer to [Password Authentication](https://www.postgresql.org/docs/18/auth-password.html) for details about migrating to another password type.
