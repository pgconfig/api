---
name: "array_nulls"
version: "18"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Enables input of NULL elements in arrays."
extra_desc: "When turned on, unquoted NULL in an array input value means a null value; otherwise it is taken literally."
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/18/runtime-config-compatible.html#GUC-ARRAY-NULLS"
---

This controls whether the array input parser recognizes unquoted `NULL` as specifying a null array element. By default, this is `on`, allowing array values containing null values to be entered. However, PostgreSQL versions before 8.2 did not support null values in arrays, and therefore would treat `NULL` as specifying a normal array element with the string value "NULL". For backward compatibility with applications that require the old behavior, this variable can be turned `off`.

Note that it is possible to create array values containing null values even when this variable is `off`.
