---
"pgconfig-api": minor
---

Add PostgreSQL 19 beta support while keeping PostgreSQL 18 as the default.

- Replace `io_workers` with the native dynamic pool defaults, `io_min_workers=2`
  and `io_max_workers=8`, for PostgreSQL 19.
- Preserve automatic analyze logging in the 19 pgBadger preset and cap the
  Windows I/O combine limit at 128kB.
- Ship all 429 parameter entries from `REL_19_BETA4` for the web app, MCP, and
  `/parameters/19/`. The legacy REST v1 `show_doc` catalog remains frozen.
- Validate the five profiles against the pinned PostgreSQL 19 beta 4 image.

Existing calculations for PostgreSQL 18 and earlier stay unchanged. New
autovacuum tuning remains outside this release.
