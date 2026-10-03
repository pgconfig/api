---
name: "debug_assertions"
version: "9.1"
type: "boolean"
category: "Developer Options"
short_desc: "Turns on various assertion checks."
extra_desc: "This is a debugging aid."
context: "user"
default: "off"
url: "https://www.postgresql.org/docs/9.1/runtime-config-developer.html#GUC-DEBUG-ASSERTIONS"
---

Turns on various assertion checks. This is a debugging aid. If you are experiencing strange problems or crashes you might want to turn this on, as it might expose programming mistakes. To use this parameter, the macro `USE_ASSERT_CHECKING` must be defined when PostgreSQL is built (accomplished by the `configure` option `--enable-cassert`). Note that `debug_assertions` defaults to `on` if PostgreSQL has been built with assertions enabled.
