---
name: "unix_socket_group"
version: "12"
type: "string"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the owning group of the Unix-domain socket."
extra_desc: "The owning user of the socket is always the user that starts the server."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/12/runtime-config-connection.html#GUC-UNIX-SOCKET-GROUP"
---

Sets the owning group of the Unix-domain socket(s). (The owning user of the sockets is always the user that starts the server.) In combination with the parameter `unix_socket_permissions` this can be used as an additional access control mechanism for Unix-domain connections. By default this is the empty string, which uses the default group of the server user. This parameter can only be set at server start.

This parameter is irrelevant on Windows, which does not have Unix-domain sockets.
