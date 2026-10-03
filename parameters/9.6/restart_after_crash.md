---
name: "restart_after_crash"
version: "9.6"
type: "boolean"
category: "Error Handling"
short_desc: "Reinitialize server after backend crash."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/9.6/runtime-config-error-handling.html#GUC-RESTART-AFTER-CRASH"
---

When set to true, which is the default, PostgreSQL will automatically reinitialize after a backend crash. Leaving this value set to true is normally the best way to maximize the availability of the database. However, in some circumstances, such as when PostgreSQL is being invoked by clusterware, it may be useful to disable the restart so that the clusterware can gain control and take any actions it deems appropriate.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
