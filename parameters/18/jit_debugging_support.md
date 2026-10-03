---
name: "jit_debugging_support"
version: "18"
type: "boolean"
category: "Developer Options"
short_desc: "Register JIT-compiled functions with debugger."
context: "superuser-backend"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-JIT-DEBUGGING-SUPPORT"
---

If LLVM has the required functionality, register generated functions with GDB. This makes debugging easier. The default setting is `off`. Only superusers and users with the appropriate `SET` privilege can change this parameter at session start, and it cannot be changed at all within a session.
