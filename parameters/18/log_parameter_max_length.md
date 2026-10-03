---
name: "log_parameter_max_length"
version: "18"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the maximum length in bytes of data logged for bind parameter values when logging statements."
extra_desc: "-1 means log values in full."
context: "superuser"
unit: "B"
default: "-1"
min: "-1"
max: "1073741823"
url: "https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-PARAMETER-MAX-LENGTH"
---

If greater than zero, each bind parameter value logged with a non-error statement-logging message is trimmed to this many bytes. Zero disables logging of bind parameters for non-error statement logs. `-1` (the default) allows bind parameters to be logged in full. If this value is specified without units, it is taken as bytes. Only superusers and users with the appropriate `SET` privilege can change this setting.

This setting only affects log messages printed as a result of [`log_statement`](https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-STATEMENT), [`log_min_duration_statement`](https://www.postgresql.org/docs/18/runtime-config-logging.html#GUC-LOG-MIN-DURATION-STATEMENT), and related settings. Non-zero values of this setting add some overhead, particularly if parameters are sent in binary form, since then conversion to text is required.
