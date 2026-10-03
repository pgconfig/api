---
name: "icu_validation_level"
version: "16"
type: "enum"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Log level for reporting invalid ICU locale strings."
context: "user"
default: "warning"
values: ["disabled", "debug5", "debug4", "debug3", "debug2", "debug1", "log", "notice", "warning", "error"]
url: "https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-ICU-VALIDATION-LEVEL"
---

When ICU locale validation problems are encountered, controls which [message level](https://www.postgresql.org/docs/16/runtime-config-logging.html#RUNTIME-CONFIG-SEVERITY-LEVELS) is used to report the problem. Valid values are `DISABLED`, `DEBUG5`, `DEBUG4`, `DEBUG3`, `DEBUG2`, `DEBUG1`, `INFO`, `NOTICE`, `WARNING`, `ERROR`, and `LOG`.

If set to `DISABLED`, does not report validation problems at all. Otherwise reports problems at the given message level. The default is `WARNING`.
