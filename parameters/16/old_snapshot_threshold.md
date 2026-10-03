---
name: "old_snapshot_threshold"
version: "16"
type: "integer"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Time before a snapshot is too old to read pages changed after the snapshot was taken."
extra_desc: "A value of -1 disables this feature."
context: "postmaster"
unit: "min"
default: "-1"
min: "-1"
max: "86400"
url: "https://www.postgresql.org/docs/16/runtime-config-resource.html#GUC-OLD-SNAPSHOT-THRESHOLD"
---

Sets the minimum amount of time that a query snapshot can be used without risk of a "snapshot too old" error occurring when using the snapshot. Data that has been dead for longer than this threshold is allowed to be vacuumed away. This can help prevent bloat in the face of snapshots which remain in use for a long time. To prevent incorrect results due to cleanup of data which would otherwise be visible to the snapshot, an error is generated when the snapshot is older than this threshold and the snapshot is used to read a page which has been modified since the snapshot was built.

If this value is specified without units, it is taken as minutes. A value of `-1` (the default) disables this feature, effectively setting the snapshot age limit to infinity. This parameter can only be set at server start.

Useful values for production work probably range from a small number of hours to a few days. Small values (such as `0` or `1min`) are only allowed because they may sometimes be useful for testing. While a setting as high as `60d` is allowed, please note that in many workloads extreme bloat or transaction ID wraparound may occur in much shorter time frames.

When this feature is enabled, freed space at the end of a relation cannot be released to the operating system, since that could remove information needed to detect the "snapshot too old" condition. All space allocated to a relation remains associated with that relation for reuse only within that relation unless explicitly freed (for example, with `VACUUM FULL`).

This setting does not attempt to guarantee that an error will be generated under any particular circumstances. In fact, if the correct results can be generated from (for example) a cursor which has materialized a result set, no error will be generated even if the underlying rows in the referenced table have been vacuumed away. Some tables cannot safely be vacuumed early, and so will not be affected by this setting, such as system catalogs. For such tables this setting will neither reduce bloat nor create a possibility of a "snapshot too old" error on scanning.
