# PostgreSQL parameter documentation

The PostgreSQL manual's entry for every configuration parameter of every major
version pgconfig supports, one file per parameter: `<version>/<name>.md`. The
file name is the parameter name in lowercase.

Each file starts with YAML front matter. Every value is a JSON string, so YAML
reads `on` and `010` as the text they are:

| Field | Meaning |
| --- | --- |
| `name` | The name as the manual writes it, such as `DateStyle` |
| `version` | The major version |
| `type` | `boolean`, `integer`, `floating point`, `string`, or `enum` |
| `category`, `short_desc`, `extra_desc` | What `pg_settings` shows |
| `context` | When a change takes effect: `postmaster` needs a restart |
| `unit` | The unit of `default`, `min`, and `max` |
| `default`, `min`, `max` | What PostgreSQL starts with and accepts, in the unit |
| `values` | The values an enum accepts |
| `url` | The entry in the PostgreSQL manual |

The manual's text follows, in Markdown. Notes and warnings are GitHub alerts.

The settings are those of a standard 64-bit Linux build. A parameter that such
a build leaves out, such as `trace_locks`, has only `name`, `version`, `type`,
and `url`.

## Where it comes from

`crates/parameter-docs` reads a PostgreSQL git checkout at the newest release
tag of each version: the text from `doc/src/sgml/config.sgml`, the settings
from the GUC tables in C. `sources.yml` records each tag. Do not edit these
files by hand: run the extraction again, as the `update-parameter-docs` skill
describes. ADR 0003 records why.

## License

The text is the PostgreSQL Global Development Group's, under the PostgreSQL
License in `COPYRIGHT`.
