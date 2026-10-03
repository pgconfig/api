---
name: "event_source"
version: "19"
type: "string"
category: "Reporting and Logging / Where to Log"
short_desc: "Sets the application name used to identify PostgreSQL messages in the event log."
context: "postmaster"
default: "PostgreSQL"
url: "https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-EVENT-SOURCE"
---

When logging to event log is enabled, this parameter determines the program name used to identify PostgreSQL messages in the log. The default is `PostgreSQL`. This parameter can only be set at server start.
