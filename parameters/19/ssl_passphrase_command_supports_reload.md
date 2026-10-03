---
name: "ssl_passphrase_command_supports_reload"
version: "19"
type: "boolean"
category: "Connections and Authentication / SSL"
short_desc: "Controls whether \"ssl_passphrase_command\" is called during server reload."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-SSL-PASSPHRASE-COMMAND-SUPPORTS-RELOAD"
---

This parameter determines whether the passphrase command set by `ssl_passphrase_command` will also be called during a configuration reload if a key file needs a passphrase. If this parameter is `off` (the default), then `ssl_passphrase_command` will be ignored during a reload and the SSL configuration will not be reloaded if a passphrase is needed. That setting is appropriate for a command that requires a TTY for prompting, which might not be available when the server is running. Setting this parameter to on might be appropriate if the passphrase is obtained from a file, for example.

This parameter must be set to `on` when running on `Windows` since all connections will perform a configuration reload due to the different process model of that platform.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
