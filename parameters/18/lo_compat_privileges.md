---
name: "lo_compat_privileges"
version: "18"
type: "boolean"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Enables backward compatibility mode for privilege checks on large objects."
extra_desc: "Skips privilege checks when reading or modifying large objects, for compatibility with PostgreSQL releases prior to 9.0."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-compatible.html#GUC-LO-COMPAT-PRIVILEGES"
---

In PostgreSQL releases prior to 9.0, large objects did not have access privileges and were, therefore, always readable and writable by all users. Setting this variable to `on` disables the new privilege checks, for compatibility with prior releases. The default is `off`. Only superusers and users with the appropriate `SET` privilege can change this setting.

Setting this variable does not disable all security checks related to large objects — only those for which the default behavior has changed in PostgreSQL 9.0.
