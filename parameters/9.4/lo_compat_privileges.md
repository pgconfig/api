---
name: "lo_compat_privileges"
version: "9.4"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Enables backward compatibility mode for privilege checks on large objects."
extra_desc: "Skips privilege checks when reading or modifying large objects, for compatibility with PostgreSQL releases prior to 9.0."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/9.4/runtime-config-compatible.html#GUC-LO-COMPAT-PRIVILEGES"
---

In PostgreSQL releases prior to 9.0, large objects did not have access privileges and were, therefore, always readable and writable by all users. Setting this variable to `on` disables the new privilege checks, for compatibility with prior releases. The default is `off`. Only superusers can change this setting.

Setting this variable does not disable all security checks related to large objects — only those for which the default behavior has changed in PostgreSQL 9.0. For example, `lo_import()` and `lo_export()` need superuser privileges regardless of this setting.
