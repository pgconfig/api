---
name: "unix_socket_directory"
version: "9.2"
type: "string"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the directory where the Unix-domain socket will be created."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/9.2/runtime-config-connection.html#GUC-UNIX-SOCKET-DIRECTORY"
---

Specifies the directory of the Unix-domain socket on which the server is to listen for connections from client applications. The default is normally `/tmp`, but can be changed at build time. This parameter can only be set at server start.

In addition to the socket file itself, which is named `.s.PGSQL.nnnn` where *nnnn* is the server's port number, an ordinary file named `.s.PGSQL.nnnn.lock` will be created in the `unix_socket_directory` directory. Neither file should ever be removed manually.

This parameter is irrelevant on Windows, which does not have Unix-domain sockets.
