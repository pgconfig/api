---
name: "log_parameter_max_length_on_error"
version: "17"
type: "integer"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the maximum length in bytes of data logged for bind parameter values when logging statements, on error."
extra_desc: "-1 to print values in full."
context: "user"
unit: "B"
default: "0"
min: "-1"
max: "1073741823"
url: "https://www.postgresql.org/docs/17/runtime-config-logging.html#GUC-LOG-PARAMETER-MAX-LENGTH-ON-ERROR"
---

If greater than zero, each bind parameter value reported in error messages is trimmed to this many bytes. Zero (the default) disables including bind parameters in error messages. `-1` allows bind parameters to be printed in full. If this value is specified without units, it is taken as bytes.

Non-zero values of this setting add overhead, as PostgreSQL will need to store textual representations of parameter values in memory at the start of each statement, whether or not an error eventually occurs. The overhead is greater when bind parameters are sent in binary form than when they are sent as text, since the former case requires data conversion while the latter only requires copying the string.
