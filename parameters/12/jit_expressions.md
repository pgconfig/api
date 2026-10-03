---
name: "jit_expressions"
version: "12"
type: "boolean"
category: "Developer Options"
short_desc: "Allow JIT compilation of expressions."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/12/runtime-config-developer.html#GUC-JIT-EXPRESSIONS"
---

Determines whether expressions are JIT compiled, when JIT compilation is activated (see [When to JIT?](https://www.postgresql.org/docs/12/jit-decision.html)). The default is `on`.
