---
name: "lc_messages"
version: "9.3"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the language in which messages are displayed."
context: "superuser"
default: ""
url: "https://www.postgresql.org/docs/9.3/runtime-config-client.html#GUC-LC-MESSAGES"
---

Sets the language in which messages are displayed. Acceptable values are system-dependent; see [Locale Support](https://www.postgresql.org/docs/9.3/locale.html) for more information. If this variable is set to the empty string (which is the default) then the value is inherited from the execution environment of the server in a system-dependent way.

On some systems, this locale category does not exist. Setting this variable will still work, but there will be no effect. Also, there is a chance that no translated messages for the desired language exist. In that case you will continue to see the English messages.

Only superusers can change this setting, because it affects the messages sent to the server log as well as to the client, and an improper value might obscure the readability of the server logs.
