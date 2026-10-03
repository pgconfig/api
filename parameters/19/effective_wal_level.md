---
name: "effective_wal_level"
version: "19"
type: "enum"
category: "Preset Options"
short_desc: "Shows effective WAL level."
context: "internal"
default: "replica"
values: ["minimal", "replica", "logical"]
url: "https://www.postgresql.org/docs/19/runtime-config-preset.html#GUC-EFFECTIVE-WAL-LEVEL"
---

Reports the actual WAL logging level currently in effect in the system. This parameter shares the same set of values as [`wal_level`](https://www.postgresql.org/docs/19/runtime-config-wal.html#GUC-WAL-LEVEL), but reflects the operational WAL level rather than the configured setting. For descriptions of possible values, refer to the `wal_level` parameter documentation.

The effective WAL level can differ from the configured `wal_level` in certain situations. For example, when `wal_level` is set to `replica` and the system has one or more logical replication slots, `effective_wal_level` will show `logical` to indicate that the system is maintaining WAL records at `logical` level equivalent.

On standby servers, `effective_wal_level` matches the value of `effective_wal_level` from the most upstream server in the replication chain.
