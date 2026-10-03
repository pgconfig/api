---
name: "escape_string_warning"
version: "12"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Warn about backslash escapes in ordinary string literals."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-compatible.html#GUC-ESCAPE-STRING-WARNING"
---

When on, a warning is issued if a backslash (`\`) appears in an ordinary string literal (`'...'` syntax) and `standard_conforming_strings` is off. The default is `on`.

Applications that wish to use backslash as escape should be modified to use escape string syntax (`E'...'`), because the default behavior of ordinary strings is now to treat backslash as an ordinary character, per SQL standard. This variable can be enabled to help locate code that needs to be changed.
