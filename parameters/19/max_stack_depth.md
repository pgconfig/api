---
name: "max_stack_depth"
version: "19"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the maximum stack depth, in kilobytes."
context: "superuser"
unit: "kB"
default: "100"
min: "100"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-MAX-STACK-DEPTH"
---

Specifies the maximum safe depth of the server's execution stack. The ideal setting for this parameter is the actual stack size limit enforced by the kernel (as set by `ulimit -s` or local equivalent), less a safety margin of a megabyte or so. The safety margin is needed because the stack depth is not checked in every routine in the server, but only in key potentially-recursive routines. If this value is specified without units, it is taken as kilobytes. The default setting is two megabytes (`2MB`), which is conservatively small and unlikely to risk crashes. However, it might be too small to allow execution of complex functions. Only superusers and users with the appropriate `SET` privilege can change this setting.

Setting `max_stack_depth` higher than the actual kernel limit will mean that a runaway recursive function can crash an individual backend process. On platforms where PostgreSQL can determine the kernel limit, the server will not allow this variable to be set to an unsafe value. However, not all platforms provide the information, so caution is recommended in selecting a value.
