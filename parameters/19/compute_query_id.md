---
name: "compute_query_id"
version: "19"
type: "enum"
category: "Statistics / Monitoring"
short_desc: "Enables in-core computation of query identifiers."
context: "superuser"
default: "auto"
values: ["auto", "regress", "on", "off"]
url: "https://www.postgresql.org/docs/19/runtime-config-statistics.html#GUC-COMPUTE-QUERY-ID"
---

Enables in-core computation of a query identifier. Query identifiers can be displayed in the [`pg_stat_activity`](https://www.postgresql.org/docs/19/monitoring-stats.html#MONITORING-PG-STAT-ACTIVITY-VIEW) view, using `EXPLAIN`, or emitted in the log if configured via the [`log_line_prefix`](https://www.postgresql.org/docs/19/runtime-config-logging.html#GUC-LOG-LINE-PREFIX) parameter. The [pg_stat_statements](https://www.postgresql.org/docs/19/pgstatstatements.html) extension also requires a query identifier to be computed. Note that an external module can alternatively be used if the in-core query identifier computation method is not acceptable. In this case, in-core computation must be always disabled. Valid values are `off` (always disabled), `on` (always enabled), `auto`, which lets modules such as [pg_stat_statements](https://www.postgresql.org/docs/19/pgstatstatements.html) automatically enable it, and `regress` which has the same effect as `auto`, except that the query identifier is not shown in the `EXPLAIN` output in order to facilitate automated regression testing. The default is `auto`.

> [!NOTE]
> To ensure that only one query identifier is calculated and displayed, extensions that calculate query identifiers should throw an error if a query identifier has already been computed.
