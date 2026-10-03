---
name: "log_line_prefix"
version: "10"
type: "string"
category: "Reporting and Logging / What to Log"
short_desc: "Controls information prefixed to each log line."
extra_desc: "If blank, no prefix is used."
context: "sighup"
default: "%m [%p] "
url: "https://www.postgresql.org/docs/10/runtime-config-logging.html#GUC-LOG-LINE-PREFIX"
---

This is a `printf`-style string that is output at the beginning of each log line. `%` characters begin "escape sequences" that are replaced with status information as outlined below. Unrecognized escapes are ignored. Other characters are copied straight to the log line. Some escapes are only recognized by session processes, and will be treated as empty by background processes such as the main server process. Status information may be aligned either left or right by specifying a numeric literal after the % and before the option. A negative value will cause the status information to be padded on the right with spaces to give it a minimum width, whereas a positive value will pad on the left. Padding can be useful to aid human readability in log files. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `'%m [%p] '` which logs a time stamp and the process ID.

| Escape | Effect | Session only |
| --- | --- | --- |
| `%a` | Application name | yes |
| `%u` | User name | yes |
| `%d` | Database name | yes |
| `%r` | Remote host name or IP address, and remote port | yes |
| `%h` | Remote host name or IP address | yes |
| `%p` | Process ID | no |
| `%t` | Time stamp without milliseconds | no |
| `%m` | Time stamp with milliseconds | no |
| `%n` | Time stamp with milliseconds (as a Unix epoch) | no |
| `%i` | Command tag: type of session's current command | yes |
| `%e` | SQLSTATE error code | no |
| `%c` | Session ID: see below | no |
| `%l` | Number of the log line for each session or process, starting at 1 | no |
| `%s` | Process start time stamp | no |
| `%v` | Virtual transaction ID (backendID/localXID) | no |
| `%x` | Transaction ID (0 if none is assigned) | no |
| `%q` | Produces no output, but tells non-session processes to stop at this point in the string; ignored by session processes | no |
| `%%` | Literal `%` | no |

The `%c` escape prints a quasi-unique session identifier, consisting of two 4-byte hexadecimal numbers (without leading zeros) separated by a dot. The numbers are the process start time and the process ID, so `%c` can also be used as a space saving way of printing those items. For example, to generate the session identifier from `pg_stat_activity`, use this query:

```
SELECT to_hex(trunc(EXTRACT(EPOCH FROM backend_start))::integer) || '.' ||
       to_hex(pid)
FROM pg_stat_activity;
```

> [!TIP]
> If you set a nonempty value for `log_line_prefix`, you should usually make its last character be a space, to provide visual separation from the rest of the log line. A punctuation character can be used too.

> [!TIP]
> Syslog produces its own time stamp and process ID information, so you probably do not want to include those escapes if you are logging to syslog.

> [!TIP]
> The `%q` escape is useful when including information that is only available in session (backend) context like user or database name. For example:
>
> ```
> log_line_prefix = '%m [%p] %q%u@%d/%a '
> ```
