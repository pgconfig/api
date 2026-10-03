---
name: "log_filename"
version: "9.6"
type: "string"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the file name pattern for log files."
context: "sighup"
default: "postgresql-%Y-%m-%d_%H%M%S.log"
url: "https://www.postgresql.org/docs/9.6/runtime-config-logging.html#GUC-LOG-FILENAME"
---

When `logging_collector` is enabled, this parameter sets the file names of the created log files. The value is treated as a `strftime` pattern, so `%`-escapes can be used to specify time-varying file names. (Note that if there are any time-zone-dependent `%`-escapes, the computation is done in the zone specified by [`log_timezone`](https://www.postgresql.org/docs/9.6/runtime-config-logging.html#GUC-LOG-TIMEZONE).) The supported `%`-escapes are similar to those listed in the Open Group's [strftime](https://pubs.opengroup.org/onlinepubs/009695399/functions/strftime.html) specification. Note that the system's `strftime` is not used directly, so platform-specific (nonstandard) extensions do not work. The default is `postgresql-%Y-%m-%d_%H%M%S.log`.

If you specify a file name without escapes, you should plan to use a log rotation utility to avoid eventually filling the entire disk. In releases prior to 8.4, if no `%` escapes were present, PostgreSQL would append the epoch of the new log file's creation time, but this is no longer the case.

If CSV-format output is enabled in `log_destination`, `.csv` will be appended to the timestamped log file name to create the file name for CSV-format output. (If `log_filename` ends in `.log`, the suffix is replaced instead.)

This parameter can only be set in the `postgresql.conf` file or on the server command line.
