---
name: "huge_pages"
version: "16"
type: "enum"
category: "Resource Usage / Memory"
short_desc: "Use of huge pages on Linux or Windows."
context: "postmaster"
default: "try"
values: ["off", "on", "try"]
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-HUGE-PAGES"
---

Controls whether huge pages are requested for the main shared memory area. Valid values are `try` (the default), `on`, and `off`. This parameter can only be set at server start. With `huge_pages` set to `try`, the server will try to request huge pages, but fall back to the default if that fails. With `on`, failure to request huge pages will prevent the server from starting up. With `off`, huge pages will not be requested.

At present, this setting is supported only on Linux and Windows. The setting is ignored on other systems when set to `try`. On Linux, it is only supported when `shared_memory_type` is set to `mmap` (the default).

The use of huge pages results in smaller page tables and less CPU time spent on memory management, increasing performance. For more details about using huge pages on Linux, see [Linux Huge Pages](https://www.postgresql.org/docs/16/kernel-resources.html#LINUX-HUGE-PAGES).

Huge pages are known as large pages on Windows. To use them, you need to assign the user right "Lock pages in memory" to the Windows user account that runs PostgreSQL. You can use Windows Group Policy tool (gpedit.msc) to assign the user right "Lock pages in memory". To start the database server on the command prompt as a standalone process, not as a Windows service, the command prompt must be run as an administrator or User Access Control (UAC) must be disabled. When the UAC is enabled, the normal command prompt revokes the user right "Lock pages in memory" when started.

Note that this setting only affects the main shared memory area. Operating systems such as Linux, FreeBSD, and Illumos can also use huge pages (also known as "super" pages or "large" pages) automatically for normal memory allocation, without an explicit request from PostgreSQL. On Linux, this is called "transparent huge pages" (THP). That feature has been known to cause performance degradation with PostgreSQL for some users on some Linux versions, so its use is currently discouraged (unlike explicit use of `huge_pages`).
