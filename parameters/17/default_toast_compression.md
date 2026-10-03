---
name: "default_toast_compression"
version: "17"
type: "enum"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the default compression method for compressible values."
context: "user"
default: "pglz"
values: ["pglz", "lz4"]
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-DEFAULT-TOAST-COMPRESSION"
---

This variable sets the default [TOAST](https://www.postgresql.org/docs/17/storage-toast.html) compression method for values of compressible columns. (This can be overridden for individual columns by setting the `COMPRESSION` column option in `CREATE TABLE` or `ALTER TABLE`.) The supported compression methods are `pglz` and (if PostgreSQL was compiled with `--with-lz4`) `lz4`. The default is `pglz`.
