---
name: "backtrace_functions"
version: "14"
type: "string"
category: "Developer Options"
short_desc: "Log backtrace for errors in these functions."
context: "superuser"
default: ""
url: "https://www.postgresql.org/docs/14/runtime-config-developer.html#GUC-BACKTRACE-FUNCTIONS"
---

This parameter contains a comma-separated list of C function names. If an error is raised and the name of the internal C function where the error happens matches a value in the list, then a backtrace is written to the server log together with the error message. This can be used to debug specific areas of the source code.

Backtrace support is not available on all platforms, and the quality of the backtraces depends on compilation options.

This parameter can only be set by superusers.
