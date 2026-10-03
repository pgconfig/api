---
name: "jit_debugging_support"
version: "13"
type: "boolean"
category: "Developer Options"
short_desc: "Register JIT compiled function with debugger."
context: "superuser-backend"
default: "off"
url: "https://www.postgresql.org/docs/13/runtime-config-developer.html#GUC-JIT-DEBUGGING-SUPPORT"
---

If LLVM has the required functionality, register generated functions with GDB. This makes debugging easier. The default setting is `off`. This parameter can only be set at server start.
