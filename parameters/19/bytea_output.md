---
name: "bytea_output"
version: "19"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the output format for bytea."
context: "user"
default: "hex"
values: ["escape", "hex"]
url: "https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-BYTEA-OUTPUT"
---

Sets the output format for values of type `bytea`. Valid values are `hex` (the default) and `escape` (the traditional PostgreSQL format). See [Binary Data Types](https://www.postgresql.org/docs/19/datatype-binary.html) for more information. The `bytea` type always accepts both formats on input, regardless of this setting.
