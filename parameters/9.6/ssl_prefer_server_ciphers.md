---
name: "ssl_prefer_server_ciphers"
version: "9.6"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Give priority to server ciphersuite order."
context: "postmaster"
default: "on"
url: "https://www.postgresql.org/docs/9.6/runtime-config-connection.html#GUC-SSL-PREFER-SERVER-CIPHERS"
---

Specifies whether to use the server's SSL cipher preferences, rather than the client's. The default is true. This parameter can only be set at server start.

Older PostgreSQL versions do not have this setting and always use the client's preferences. This setting is mainly for backward compatibility with those versions. Using the server's preferences is usually better because it is more likely that the server is appropriately configured.
