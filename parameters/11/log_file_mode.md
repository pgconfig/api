---
name: "log_file_mode"
version: "11"
type: "integer"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the file permissions for log files."
extra_desc: "The parameter value is expected to be a numeric mode specification in the form accepted by the chmod and umask system calls. (To use the customary octal format the number must start with a 0 (zero).)"
context: "sighup"
default: "384"
min: "0"
max: "511"
url: "https://www.postgresql.org/docs/11/runtime-config-logging.html#GUC-LOG-FILE-MODE"
---

On Unix systems this parameter sets the permissions for log files when `logging_collector` is enabled. (On Microsoft Windows this parameter is ignored.) The parameter value is expected to be a numeric mode specified in the format accepted by the `chmod` and `umask` system calls. (To use the customary octal format the number must start with a `0` (zero).)

The default permissions are `0600`, meaning only the server owner can read or write the log files. The other commonly useful setting is `0640`, allowing members of the owner's group to read the files. Note however that to make use of such a setting, you'll need to alter [`log_directory`](https://www.postgresql.org/docs/11/runtime-config-logging.html#GUC-LOG-DIRECTORY) to store the files somewhere outside the cluster data directory. In any case, it's unwise to make the log files world-readable, since they might contain sensitive data.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
