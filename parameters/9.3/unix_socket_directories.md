---
name: "unix_socket_directories"
version: "9.3"
type: "string"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the directories where Unix-domain sockets will be created."
context: "postmaster"
default: "/tmp"
url: "https://www.postgresql.org/docs/9.3/runtime-config-connection.html#GUC-UNIX-SOCKET-DIRECTORIES"
---

Specifies the directory of the Unix-domain socket(s) on which the server is to listen for connections from client applications. Multiple sockets can be created by listing multiple directories separated by commas. Whitespace between entries is ignored; surround a directory name with double quotes if you need to include whitespace or commas in the name. An empty value specifies not listening on any Unix-domain sockets, in which case only TCP/IP sockets can be used to connect to the server. The default value is normally `/tmp`, but that can be changed at build time. This parameter can only be set at server start.

In addition to the socket file itself, which is named `.s.PGSQL.nnnn` where *nnnn* is the server's port number, an ordinary file named `.s.PGSQL.nnnn.lock` will be created in each of the `unix_socket_directories` directories. Neither file should ever be removed manually.

This parameter is irrelevant on Windows, which does not have Unix-domain sockets.
