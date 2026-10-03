---
name: "jit_expressions"
version: "13"
type: "boolean"
category: "Developer Options"
short_desc: "Allow JIT compilation of expressions."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/13/runtime-config-developer.html#GUC-JIT-EXPRESSIONS"
---

Determines whether expressions are JIT compiled, when JIT compilation is activated (see [When to JIT?](https://www.postgresql.org/docs/13/jit-decision.html)). The default is `on`.
