---
name: "port"
version: "9.6"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the TCP port the server listens on."
context: "postmaster"
default: "5432"
min: "1"
max: "65535"
url: "https://www.postgresql.org/docs/9.6/runtime-config-connection.html#GUC-PORT"
---

The TCP port the server listens on; 5432 by default. Note that the same port number is used for all IP addresses the server listens on. This parameter can only be set at server start.
