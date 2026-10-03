---
name: "log_parameter_max_length"
version: "13"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "When logging statements, limit logged parameter values to first N bytes."
extra_desc: "-1 to print values in full."
context: "superuser"
unit: "B"
default: "-1"
min: "-1"
max: "1073741823"
url: "https://www.postgresql.org/docs/13/runtime-config-logging.html#GUC-LOG-PARAMETER-MAX-LENGTH"
---

If greater than zero, each bind parameter value logged with a non-error statement-logging message is trimmed to this many bytes. Zero disables logging of bind parameters for non-error statement logs. `-1` (the default) allows bind parameters to be logged in full. If this value is specified without units, it is taken as bytes. Only superusers can change this setting.

This setting only affects log messages printed as a result of [`log_statement`](https://www.postgresql.org/docs/13/runtime-config-logging.html#GUC-LOG-STATEMENT), [`log_duration`](https://www.postgresql.org/docs/13/runtime-config-logging.html#GUC-LOG-DURATION), and related settings. Non-zero values of this setting add some overhead, particularly if parameters are sent in binary form, since then conversion to text is required.
