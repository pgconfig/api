---
name: "ssl_passphrase_command"
version: "15"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Command to obtain passphrases for SSL."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/15/runtime-config-connection.html#GUC-SSL-PASSPHRASE-COMMAND"
---

Sets an external command to be invoked when a passphrase for decrypting an SSL file such as a private key needs to be obtained. By default, this parameter is empty, which means the built-in prompting mechanism is used.

The command must print the passphrase to the standard output and exit with code 0. In the parameter value, `%p` is replaced by a prompt string. (Write `%%` for a literal `%`.) Note that the prompt string will probably contain whitespace, so be sure to quote adequately. A single newline is stripped from the end of the output if present.

The command does not actually have to prompt the user for a passphrase. It can read it from a file, obtain it from a keychain facility, or similar. It is up to the user to make sure the chosen mechanism is adequately secure.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
