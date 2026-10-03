---
name: "exit_on_error"
version: "12"
type: "boolean"
category: "Error Handling"
short_desc: "Terminate session on any error."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/12/runtime-config-error-handling.html#GUC-EXIT-ON-ERROR"
---

If on, any error will terminate the current session. By default, this is set to off, so that only FATAL errors will terminate the session.
