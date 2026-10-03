---
name: "stats_temp_directory"
version: "9.2"
type: "string"
category: "Statistics / Query and Index Statistics Collector"
short_desc: "Writes temporary statistics files to the specified directory."
context: "sighup"
default: "pg_stat_tmp"
url: "https://www.postgresql.org/docs/9.2/runtime-config-statistics.html#GUC-STATS-TEMP-DIRECTORY"
---

Sets the directory to store temporary statistics data in. This can be a path relative to the data directory or an absolute path. The default is `pg_stat_tmp`. Pointing this at a RAM-based file system will decrease physical I/O requirements and can lead to improved performance. This parameter can only be set in the `postgresql.conf` file or on the server command line.
