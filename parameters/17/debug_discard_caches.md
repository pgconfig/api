---
name: "debug_discard_caches"
version: "17"
type: "integer"
category: "Developer Options"
short_desc: "Aggressively flush system caches for debugging purposes."
context: "superuser"
default: "0"
min: "0"
max: "0"
url: "https://www.postgresql.org/docs/17/runtime-config-developer.html#GUC-DEBUG-DISCARD-CACHES"
---

When set to `1`, each system catalog cache entry is invalidated at the first possible opportunity, whether or not anything that would render it invalid really occurred. Caching of system catalogs is effectively disabled as a result, so the server will run extremely slowly. Higher values run the cache invalidation recursively, which is even slower and only useful for testing the caching logic itself. The default value of `0` selects normal catalog caching behavior.

This parameter can be very helpful when trying to trigger hard-to-reproduce bugs involving concurrent catalog changes, but it is otherwise rarely needed. See the source code files `inval.c` and `pg_config_manual.h` for details.

This parameter is supported when `DISCARD_CACHES_ENABLED` was defined at compile time (which happens automatically when using the configure option `--enable-cassert`). In production builds, its value will always be `0` and attempts to set it to another value will raise an error.
