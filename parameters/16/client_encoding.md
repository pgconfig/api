---
name: "client_encoding"
version: "16"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the client's character set encoding."
context: "user"
default: "SQL_ASCII"
url: "https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-CLIENT-ENCODING"
---

Sets the client-side encoding (character set). The default is to use the database encoding. The character sets supported by the PostgreSQL server are described in [Supported Character Sets](https://www.postgresql.org/docs/16/multibyte.html#MULTIBYTE-CHARSET-SUPPORTED).
