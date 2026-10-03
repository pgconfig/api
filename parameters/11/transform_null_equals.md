---
name: "transform_null_equals"
version: "11"
type: "boolean"
category: "Version and Platform Compatibility / Other Platforms and Clients"
short_desc: "Treats \"expr=NULL\" as \"expr IS NULL\"."
extra_desc: "When turned on, expressions of the form expr = NULL (or NULL = expr) are treated as expr IS NULL, that is, they return true if expr evaluates to the null value, and false otherwise. The correct behavior of expr = NULL is to always return null (unknown)."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/11/runtime-config-compatible.html#GUC-TRANSFORM-NULL-EQUALS"
---

When on, expressions of the form `expr = NULL` (or `NULL = expr`) are treated as `expr IS NULL`, that is, they return true if *expr* evaluates to the null value, and false otherwise. The correct SQL-spec-compliant behavior of `expr = NULL` is to always return null (unknown). Therefore this parameter defaults to `off`.

However, filtered forms in Microsoft Access generate queries that appear to use `expr = NULL` to test for null values, so if you use that interface to access the database you might want to turn this option on. Since expressions of the form `expr = NULL` always return the null value (using the SQL standard interpretation), they are not very useful and do not appear often in normal applications so this option does little harm in practice. But new users are frequently confused about the semantics of expressions involving null values, so this option is off by default.

Note that this option only affects the exact form `= NULL`, not other comparison operators or other expressions that are computationally equivalent to some expression involving the equals operator (such as `IN`). Thus, this option is not a general fix for bad programming.

Refer to [Comparison Functions and Operators](https://www.postgresql.org/docs/11/functions-comparison.html) for related information.
