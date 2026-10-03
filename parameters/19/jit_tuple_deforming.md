---
name: "jit_tuple_deforming"
version: "19"
type: "boolean"
category: "Developer Options"
short_desc: "Allow JIT compilation of tuple deforming."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-JIT-TUPLE-DEFORMING"
---

Determines whether tuple deforming is JIT compiled, when JIT compilation is activated (see [When to JIT?](https://www.postgresql.org/docs/19/jit-decision.html)). The default is `on`.
