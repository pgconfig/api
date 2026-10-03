---
name: "wal_consistency_checking"
version: "19"
type: "string"
category: "Developer Options"
short_desc: "Sets the WAL resource managers for which WAL consistency checks are done."
extra_desc: "Full-page images will be logged for all data blocks and cross-checked against the results of WAL replay."
context: "superuser"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-WAL-CONSISTENCY-CHECKING"
---

This parameter is intended to be used to check for bugs in the WAL redo routines. When enabled, full-page images of any buffers modified in conjunction with the WAL record are added to the record. If the record is subsequently replayed, the system will first apply each record and then test whether the buffers modified by the record match the stored images. In certain cases (such as hint bits), minor variations are acceptable, and will be ignored. Any unexpected differences will result in a fatal error, terminating recovery.

The default value of this setting is the empty string, which disables the feature. It can be set to `all` to check all records, or to a comma-separated list of resource managers to check only records originating from those resource managers. Currently, the supported resource managers are `heap`, `heap2`, `btree`, `hash`, `gin`, `gist`, `sequence`, `spgist`, `brin`, and `generic`. Extensions may define additional resource managers. Only superusers and users with the appropriate `SET` privilege can change this setting.
