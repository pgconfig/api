---
name: "data_directory_mode"
version: "18"
type: "integer"
category: "Preset Options"
short_desc: "Shows the mode of the data directory."
extra_desc: "The parameter value is a numeric mode specification in the form accepted by the chmod and umask system calls. (To use the customary octal format the number must start with a 0 (zero).)"
context: "internal"
default: "448"
min: "0"
max: "511"
url: "https://www.postgresql.org/docs/18/runtime-config-preset.html#GUC-DATA-DIRECTORY-MODE"
---

On Unix systems this parameter reports the permissions the data directory (defined by [`data_directory`](https://www.postgresql.org/docs/18/runtime-config-file-locations.html#GUC-DATA-DIRECTORY)) had at server startup. (On Microsoft Windows this parameter will always display `0700`.) See [the initdb `-g` option](https://www.postgresql.org/docs/18/app-initdb.html#APP-INITDB-ALLOW-GROUP-ACCESS) for more information.
