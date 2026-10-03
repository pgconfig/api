---
name: "log_destination"
version: "17"
type: "string"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the destination for server log output."
extra_desc: "Valid values are combinations of \"stderr\", \"syslog\", \"csvlog\", \"jsonlog\", and \"eventlog\", depending on the platform."
context: "sighup"
default: "stderr"
url: "https://www.postgresql.org/docs/17/runtime-config-logging.html#GUC-LOG-DESTINATION"
---

PostgreSQL supports several methods for logging server messages, including `stderr`, `csvlog`, `jsonlog`, and `syslog`. On Windows, `eventlog` is also supported. Set this parameter to a list of desired log destinations separated by commas. The default is to log to `stderr` only. This parameter can only be set in the `postgresql.conf` file or on the server command line.

If `csvlog` is included in `log_destination`, log entries are output in "comma separated value" (CSV) format, which is convenient for loading logs into programs. See [Using CSV-Format Log Output](https://www.postgresql.org/docs/17/runtime-config-logging.html#RUNTIME-CONFIG-LOGGING-CSVLOG) for details. [`logging_collector`](https://www.postgresql.org/docs/17/runtime-config-logging.html#GUC-LOGGING-COLLECTOR) must be enabled to generate CSV-format log output.

If `jsonlog` is included in `log_destination`, log entries are output in JSON format, which is convenient for loading logs into programs. See [Using JSON-Format Log Output](https://www.postgresql.org/docs/17/runtime-config-logging.html#RUNTIME-CONFIG-LOGGING-JSONLOG) for details. [`logging_collector`](https://www.postgresql.org/docs/17/runtime-config-logging.html#GUC-LOGGING-COLLECTOR) must be enabled to generate JSON-format log output.

When either `stderr`, `csvlog` or `jsonlog` are included, the file `current_logfiles` is created to record the location of the log file(s) currently in use by the logging collector and the associated logging destination. This provides a convenient way to find the logs currently in use by the instance. Here is an example of this file's content:

```
stderr log/postgresql.log
csvlog log/postgresql.csv
jsonlog log/postgresql.json
```

`current_logfiles` is recreated when a new log file is created as an effect of rotation, and when `log_destination` is reloaded. It is removed when none of `stderr`, `csvlog` or `jsonlog` are included in `log_destination`, and when the logging collector is disabled.

> [!NOTE]
> On most Unix systems, you will need to alter the configuration of your system's syslog daemon in order to make use of the `syslog` option for `log_destination`. PostgreSQL can log to syslog facilities `LOCAL0` through `LOCAL7` (see [`syslog_facility`](https://www.postgresql.org/docs/17/runtime-config-logging.html#GUC-SYSLOG-FACILITY)), but the default syslog configuration on most platforms will discard all such messages. You will need to add something like:
>
> ```
> local0.*    /var/log/postgresql
> ```
>
> to the syslog daemon's configuration file to make it work.
>
> On Windows, when you use the `eventlog` option for `log_destination`, you should register an event source and its library with the operating system so that the Windows Event Viewer can display event log messages cleanly. See [Registering Event Log on Windows](https://www.postgresql.org/docs/17/event-log-registration.html) for details.
