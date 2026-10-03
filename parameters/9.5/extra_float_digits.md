---
name: "extra_float_digits"
version: "9.5"
type: "integer"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the number of digits displayed for floating-point values."
extra_desc: "This affects real, double precision, and geometric data types. The parameter value is added to the standard number of digits (FLT_DIG or DBL_DIG as appropriate)."
context: "user"
default: "0"
min: "-15"
max: "3"
url: "https://www.postgresql.org/docs/9.5/runtime-config-client.html#GUC-EXTRA-FLOAT-DIGITS"
---

This parameter adjusts the number of digits displayed for floating-point values, including `float4`, `float8`, and geometric data types. The parameter value is added to the standard number of digits (`FLT_DIG` or `DBL_DIG` as appropriate). The value can be set as high as 3, to include partially-significant digits; this is especially useful for dumping float data that needs to be restored exactly. Or it can be set negative to suppress unwanted digits. See also [Floating-Point Types](https://www.postgresql.org/docs/9.5/datatype-numeric.html#DATATYPE-FLOAT).
