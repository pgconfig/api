---
name: "bonjour_name"
version: "9.2"
type: "string"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the Bonjour service name."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/9.2/runtime-config-connection.html#GUC-BONJOUR-NAME"
---

Specifies the Bonjour service name. The computer name is used if this parameter is set to the empty string `''` (which is the default). This parameter is ignored if the server was not compiled with Bonjour support. This parameter can only be set at server start.
