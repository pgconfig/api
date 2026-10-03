---
name: "log_destination"
version: "9.1"
type: "string"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the destination for server log output."
extra_desc: "Valid values are combinations of \"stderr\", \"syslog\", \"csvlog\", and \"eventlog\", depending on the platform."
context: "sighup"
default: "stderr"
url: "https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-LOG-DESTINATION"
---

PostgreSQL supports several methods for logging server messages, including `stderr`, `csvlog` and `syslog`. On Windows, `eventlog` is also supported. Set this parameter to a list of desired log destinations separated by commas. The default is to log to `stderr` only. This parameter can only be set in the `postgresql.conf` file or on the server command line.

If `csvlog` is included in `log_destination`, log entries are output in "comma separated value" (CSV) format, which is convenient for loading logs into programs. See [Using CSV-Format Log Output](https://www.postgresql.org/docs/9.1/runtime-config-logging.html#RUNTIME-CONFIG-LOGGING-CSVLOG) for details. [`logging_collector`](https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-LOGGING-COLLECTOR) must be enabled to generate CSV-format log output.

> [!NOTE]
> On most Unix systems, you will need to alter the configuration of your system's syslog daemon in order to make use of the `syslog` option for `log_destination`. PostgreSQL can log to syslog facilities `LOCAL0` through `LOCAL7` (see [`syslog_facility`](https://www.postgresql.org/docs/9.1/runtime-config-logging.html#GUC-SYSLOG-FACILITY)), but the default syslog configuration on most platforms will discard all such messages. You will need to add something like:
>
> ```
> local0.*    /var/log/postgresql
> ```
>
> to the syslog daemon's configuration file to make it work.
