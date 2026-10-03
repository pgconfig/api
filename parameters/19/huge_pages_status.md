---
name: "huge_pages_status"
version: "19"
type: "enum"
category: "Preset Options"
short_desc: "Indicates the status of huge pages."
context: "internal"
default: "unknown"
values: ["off", "on", "unknown"]
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-HUGE-PAGES-STATUS"
---

Reports the state of huge pages in the current instance: `on`, `off`, or `unknown` (if displayed with `postgres -C`). This parameter is useful to determine whether allocation of huge pages was successful under `huge_pages=try`. See [`huge_pages`](https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-HUGE-PAGES) for more information.
