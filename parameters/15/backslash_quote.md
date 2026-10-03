---
name: "backslash_quote"
version: "15"
type: "enum"
category: "Version and Platform Compatibility / Previous PostgreSQL Versions"
short_desc: "Sets whether \"\\'\" is allowed in string literals."
context: "user"
default: "safe_encoding"
values: ["safe_encoding", "on", "off"]
url: "https://www.postgresql.org/docs/15/runtime-config-compatible.html#GUC-BACKSLASH-QUOTE"
---

This controls whether a quote mark can be represented by `\'` in a string literal. The preferred, SQL-standard way to represent a quote mark is by doubling it (`''`) but PostgreSQL has historically also accepted `\'`. However, use of `\'` creates security risks because in some client character set encodings, there are multibyte characters in which the last byte is numerically equivalent to ASCII `\`. If client-side code does escaping incorrectly then an SQL-injection attack is possible. This risk can be prevented by making the server reject queries in which a quote mark appears to be escaped by a backslash. The allowed values of `backslash_quote` are `on` (allow `\'` always), `off` (reject always), and `safe_encoding` (allow only if client encoding does not allow ASCII `\` within a multibyte character). `safe_encoding` is the default setting.

Note that in a standard-conforming string literal, `\` just means `\` anyway. This parameter only affects the handling of non-standard-conforming literals, including escape string syntax (`E'...'`).
