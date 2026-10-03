---
name: "huge_page_size"
version: "19"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "The size of huge page that should be requested."
extra_desc: "0 means use the system default."
context: "postmaster"
unit: "kB"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-HUGE-PAGE-SIZE"
---

Controls the size of huge pages, when they are enabled with [`huge_pages`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-HUGE-PAGES). The default is zero (`0`). When set to `0`, the default huge page size on the system will be used. This parameter can only be set at server start.

Some commonly available page sizes on modern 64 bit server architectures include: `2MB` and `1GB` (Intel and AMD), `16MB` and `16GB` (IBM POWER), and `64kB`, `2MB`, `32MB` and `1GB` (ARM). For more information about usage and support, see [Linux Huge Pages](https://www.postgresql.org/docs/19/kernel-resources.html#LINUX-HUGE-PAGES).

Non-default settings are currently supported only on Linux.
