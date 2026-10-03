---
name: "wal_log_hints"
version: "13"
type: "boolean"
category: "Write-Ahead Log / Settings"
short_desc: "Writes full pages to WAL when first modified after a checkpoint, even for a non-critical modification."
context: "postmaster"
default: "off"
url: "https://www.postgresql.org/docs/13/runtime-config-wal.html#GUC-WAL-LOG-HINTS"
---

When this parameter is `on`, the PostgreSQL server writes the entire content of each disk page to WAL during the first modification of that page after a checkpoint, even for non-critical modifications of so-called hint bits.

If data checksums are enabled, hint bit updates are always WAL-logged and this setting is ignored. You can use this setting to test how much extra WAL-logging would occur if your database had data checksums enabled.

This parameter can only be set at server start. The default value is `off`.
