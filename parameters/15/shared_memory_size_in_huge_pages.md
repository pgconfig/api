---
name: "shared_memory_size_in_huge_pages"
version: "15"
type: "integer"
category: "Preset Options"
short_desc: "Shows the number of huge pages needed for the main shared memory area."
extra_desc: "-1 indicates that the value could not be determined."
context: "internal"
default: "-1"
min: "-1"
max: "2147483647"
url: "https://www.postgresql.org/docs/15/runtime-config-preset.html#GUC-SHARED-MEMORY-SIZE-IN-HUGE-PAGES"
---

Reports the number of huge pages that are needed for the main shared memory area based on the specified [`huge_page_size`](https://www.postgresql.org/docs/15/runtime-config-resource.html#GUC-HUGE-PAGE-SIZE). If huge pages are not supported, this will be `-1`.

This setting is supported only on Linux. It is always set to `-1` on other platforms. For more details about using huge pages on Linux, see [Linux Huge Pages](https://www.postgresql.org/docs/15/kernel-resources.html#LINUX-HUGE-PAGES).
