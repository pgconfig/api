---
name: "extra_float_digits"
version: "19"
type: "integer"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the number of digits displayed for floating-point values."
extra_desc: "This affects real, double precision, and geometric data types. A zero or negative parameter value is added to the standard number of digits (FLT_DIG or DBL_DIG as appropriate). Any value greater than zero selects precise output mode."
context: "user"
default: "1"
min: "-15"
max: "3"
url: "https://www.postgresql.org/docs/19/runtime-config-client.html#GUC-EXTRA-FLOAT-DIGITS"
---

This parameter adjusts the number of digits used for textual output of floating-point values, including `float4`, `float8`, and geometric data types.

If the value is 1 (the default) or above, float values are output in shortest-precise format; see [Floating-Point Types](https://www.postgresql.org/docs/19/datatype-numeric.html#DATATYPE-FLOAT). The actual number of digits generated depends only on the value being output, not on the value of this parameter. At most 17 digits are required for `float8` values, and 9 for `float4` values. This format is both fast and precise, preserving the original binary float value exactly when correctly read. For historical compatibility, values up to 3 are permitted.

If the value is zero or negative, then the output is rounded to a given decimal precision. The precision used is the standard number of digits for the type (`FLT_DIG` or `DBL_DIG` as appropriate) reduced according to the value of this parameter. (For example, specifying -1 will cause `float4` values to be output rounded to 5 significant digits, and `float8` values rounded to 14 digits.) This format is slower and does not preserve all the bits of the binary float value, but may be more human-readable.

> [!NOTE]
> The meaning of this parameter, and its default value, changed in PostgreSQL 12; see [Floating-Point Types](https://www.postgresql.org/docs/19/datatype-numeric.html#DATATYPE-FLOAT) for further discussion.
