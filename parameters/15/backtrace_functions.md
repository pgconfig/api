---
name: "backtrace_functions"
version: "15"
type: "string"
category: "Developer Options"
short_desc: "Log backtrace for errors in these functions."
context: "superuser"
default: ""
url: "https://www.postgresql.org/docs/15/runtime-config-developer.html#GUC-BACKTRACE-FUNCTIONS"
---

This parameter contains a comma-separated list of C function names. If an error is raised and the name of the internal C function where the error happens matches a value in the list, then a backtrace is written to the server log together with the error message. This can be used to debug specific areas of the source code.

Backtrace support is not available on all platforms, and the quality of the backtraces depends on compilation options.

Only superusers and users with the appropriate `SET` privilege can change this setting.
