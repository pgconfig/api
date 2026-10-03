---
name: "recovery_target_time"
version: "12"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Sets the time stamp up to which recovery will proceed."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/12/runtime-config-wal.html#GUC-RECOVERY-TARGET-TIME"
---

This parameter specifies the time stamp up to which recovery will proceed. The precise stopping point is also influenced by [`recovery_target_inclusive`](https://www.postgresql.org/docs/12/runtime-config-wal.html#GUC-RECOVERY-TARGET-INCLUSIVE).

The value of this parameter is a time stamp in the same format accepted by the `timestamp with time zone` data type, except that you cannot use a time zone abbreviation (unless the [`timezone_abbreviations`](https://www.postgresql.org/docs/12/runtime-config-client.html#GUC-TIMEZONE-ABBREVIATIONS) variable has been set earlier in the configuration file). Preferred style is to use a numeric offset from UTC, or you can write a full time zone name, e.g., `Europe/Helsinki` not `EEST`.
