---
name: "xmloption"
version: "9.6"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets whether XML data in implicit parsing and serialization operations is to be considered as documents or content fragments."
context: "user"
default: "content"
values: ["content", "document"]
url: "https://www.postgresql.org/docs/9.6/runtime-config-client.html#GUC-XMLOPTION"
---

Sets whether `DOCUMENT` or `CONTENT` is implicit when converting between XML and character string values. See [XML Type](https://www.postgresql.org/docs/9.6/datatype-xml.html) for a description of this. Valid values are `DOCUMENT` and `CONTENT`. The default is `CONTENT`.

According to the SQL standard, the command to set this option is

```
SET XML OPTION { DOCUMENT | CONTENT };
```

This syntax is also available in PostgreSQL.
