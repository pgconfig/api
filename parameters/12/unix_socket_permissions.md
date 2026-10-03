---
name: "unix_socket_permissions"
version: "12"
type: "integer"
category: "Connections and Authentication / Connection Settings"
short_desc: "Sets the access permissions of the Unix-domain socket."
extra_desc: "Unix-domain sockets use the usual Unix file system permission set. The parameter value is expected to be a numeric mode specification in the form accepted by the chmod and umask system calls. (To use the customary octal format the number must start with a 0 (zero).)"
context: "postmaster"
default: "511"
min: "0"
max: "511"
url: "https://www.postgresql.org/docs/12/runtime-config-connection.html#GUC-UNIX-SOCKET-PERMISSIONS"
---

Sets the access permissions of the Unix-domain socket(s). Unix-domain sockets use the usual Unix file system permission set. The parameter value is expected to be a numeric mode specified in the format accepted by the `chmod` and `umask` system calls. (To use the customary octal format the number must start with a `0` (zero).)

The default permissions are `0777`, meaning anyone can connect. Reasonable alternatives are `0770` (only user and group, see also `unix_socket_group`) and `0700` (only user). (Note that for a Unix-domain socket, only write permission matters, so there is no point in setting or revoking read or execute permissions.)

This access control mechanism is independent of the one described in [Client Authentication](https://www.postgresql.org/docs/12/client-authentication.html).

This parameter can only be set at server start.

This parameter is irrelevant on systems, notably Solaris as of Solaris 10, that ignore socket permissions entirely. There, one can achieve a similar effect by pointing `unix_socket_directories` to a directory having search permission limited to the desired audience. This parameter is also irrelevant on Windows, which does not have Unix-domain sockets.
