---
name: "huge_pages"
version: "9.4"
type: "enum"
category: "Resource Usage / Memory"
short_desc: "Use of huge pages on Linux."
context: "postmaster"
default: "try"
values: ["off", "on", "try"]
url: "https://www.postgresql.org/docs/9.4/runtime-config-resource.html#GUC-HUGE-PAGES"
---

Enables/disables the use of huge memory pages. Valid values are `try` (the default), `on`, and `off`.

At present, this feature is supported only on Linux. The setting is ignored on other systems when set to `try`.

The use of huge pages results in smaller page tables and less CPU time spent on memory management, increasing performance. For more details, see [Linux Huge Pages](https://www.postgresql.org/docs/9.4/kernel-resources.html#LINUX-HUGE-PAGES).

With `huge_pages` set to `try`, the server will try to use huge pages, but fall back to using normal allocation if that fails. With `on`, failure to use huge pages will prevent the server from starting up. With `off`, huge pages will not be used.
