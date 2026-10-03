---
name: "jit_provider"
version: "14"
type: "string"
category: "Client Connection Defaults / Shared Library Preloading"
short_desc: "JIT provider to use."
context: "postmaster"
default: "llvmjit"
url: "https://www.postgresql.org/docs/14/runtime-config-client.html#GUC-JIT-PROVIDER"
---

This variable is the name of the JIT provider library to be used (see [Pluggable JIT Providers](https://www.postgresql.org/docs/14/jit-extensibility.html#JIT-PLUGGABLE)). The default is `llvmjit`. This parameter can only be set at server start.

If set to a non-existent library, JIT will not be available, but no error will be raised. This allows JIT support to be installed separately from the main PostgreSQL package.
