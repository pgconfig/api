---
name: "log_statement"
version: "9.4"
type: "enum"
category: "Reporting and Logging / What to Log"
short_desc: "Sets the type of statements logged."
context: "superuser"
default: "none"
values: ["none", "ddl", "mod", "all"]
url: "https://www.postgresql.org/docs/9.4/runtime-config-logging.html#GUC-LOG-STATEMENT"
---

Controls which SQL statements are logged. Valid values are `none` (off), `ddl`, `mod`, and `all` (all statements). `ddl` logs all data definition statements, such as `CREATE`, `ALTER`, and `DROP` statements. `mod` logs all `ddl` statements, plus data-modifying statements such as `INSERT`, `UPDATE`, `DELETE`, `TRUNCATE`, and `COPY FROM`. `PREPARE`, `EXECUTE`, and `EXPLAIN ANALYZE` statements are also logged if their contained command is of an appropriate type. For clients using extended query protocol, logging occurs when an Execute message is received, and values of the Bind parameters are included (with any embedded single-quote marks doubled).

The default is `none`. Only superusers can change this setting.

> [!NOTE]
> Statements that contain simple syntax errors are not logged even by the `log_statement` = `all` setting, because the log message is emitted only after basic parsing has been done to determine the statement type. In the case of extended query protocol, this setting likewise does not log statements that fail before the Execute phase (i.e., during parse analysis or planning). Set `log_min_error_statement` to `ERROR` (or lower) to log such statements.
