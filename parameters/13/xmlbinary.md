---
name: "xmlbinary"
version: "13"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets how binary values are to be encoded in XML."
context: "user"
default: "base64"
values: ["base64", "hex"]
url: "https://www.postgresql.org/docs/13/runtime-config-client.html#GUC-XMLBINARY"
---

Sets how binary values are to be encoded in XML. This applies for example when `bytea` values are converted to XML by the functions `xmlelement` or `xmlforest`. Possible values are `base64` and `hex`, which are both defined in the XML Schema standard. The default is `base64`. For further information about XML-related functions, see [XML Functions](https://www.postgresql.org/docs/13/functions-xml.html).

The actual choice here is mostly a matter of taste, constrained only by possible restrictions in client applications. Both methods support all possible values, although the hex encoding will be somewhat larger than the base64 encoding.
