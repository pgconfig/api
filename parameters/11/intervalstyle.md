---
name: "IntervalStyle"
version: "11"
type: "enum"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the display format for interval values."
context: "user"
default: "postgres"
values: ["postgres", "postgres_verbose", "sql_standard", "iso_8601"]
url: "https://www.postgresql.org/docs/11/runtime-config-client.html#GUC-INTERVALSTYLE"
---

Sets the display format for interval values. The value `sql_standard` will produce output matching SQL standard interval literals. The value `postgres` (which is the default) will produce output matching PostgreSQL releases prior to 8.4 when the [`DateStyle`](https://www.postgresql.org/docs/11/runtime-config-client.html#GUC-DATESTYLE) parameter was set to `ISO`. The value `postgres_verbose` will produce output matching PostgreSQL releases prior to 8.4 when the `DateStyle` parameter was set to non-`ISO` output. The value `iso_8601` will produce output matching the time interval "format with designators" defined in section 4.4.3.2 of ISO 8601.

The `IntervalStyle` parameter also affects the interpretation of ambiguous interval input. See [Interval Input](https://www.postgresql.org/docs/11/datatype-datetime.html#DATATYPE-INTERVAL-INPUT) for more information.
