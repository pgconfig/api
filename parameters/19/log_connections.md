---
name: "log_connections"
version: "19"
type: "string"
category: "Reporting and Logging / What to Log"
short_desc: "Logs specified aspects of connection establishment and setup."
context: "superuser-backend"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-CONNECTIONS"
---

Causes aspects of each connection to the server to be logged. The default is the empty string, `''`, which disables all connection logging. The following options may be specified alone or in a comma-separated list:

**Log Connection Options**

| Name | Description |
| --- | --- |
| `receipt` | Logs receipt of a connection. |
| `authentication` | Logs the original identity used by an authentication method to identify a user. In most cases, the identity string matches the PostgreSQL username, but some third-party authentication methods may alter the original user identifier before the server stores it. Failed authentication is always logged regardless of the value of this setting. |
| `authorization` | Logs successful completion of authorization. At this point the connection has been established but the backend is not yet fully set up. The log message includes the authorized username as well as the database name and application name, if applicable. |
| `setup_durations` | Logs the time spent establishing the connection and setting up the backend until the connection is ready to execute its first query. The log message includes three durations: the total setup duration (starting from the postmaster accepting the incoming connection and ending when the connection is ready for query), the time it took to fork the new backend, and the time it took to authenticate the user. |
| `all` | A convenience alias equivalent to specifying all options. If `all` is specified in a list of other options, all connection aspects will be logged. |

Disconnection logging is separately controlled by [`log_disconnections`](https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-DISCONNECTIONS).

For the purposes of backwards compatibility, `on`, `off`, `true`, `false`, `yes`, `no`, `1`, and `0` are still supported. The positive values are equivalent to specifying the `receipt`, `authentication`, and `authorization` options.

Only superusers and users with the appropriate `SET` privilege can change this parameter at session start, and it cannot be changed at all within a session.

> [!NOTE]
> Some client programs, like psql, attempt to connect twice while determining if a password is required, so duplicate "connection received" messages do not necessarily indicate a problem.
