---
name: "exit_on_error"
version: "9.6"
type: "boolean"
category: "Error Handling"
short_desc: "Terminate session on any error."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/9.6/runtime-config-error-handling.html#GUC-EXIT-ON-ERROR"
---

If true, any error will terminate the current session. By default, this is set to false, so that only FATAL errors will terminate the session.
