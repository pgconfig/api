---
name: "DateStyle"
version: "10"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets the display format for date and time values."
extra_desc: "Also controls interpretation of ambiguous date inputs."
context: "user"
default: "ISO, MDY"
url: "https://www.postgresql.org/docs/10/runtime-config-client.html#GUC-DATESTYLE"
---

Sets the display format for date and time values, as well as the rules for interpreting ambiguous date input values. For historical reasons, this variable contains two independent components: the output format specification (`ISO`, `Postgres`, `SQL`, or `German`) and the input/output specification for year/month/day ordering (`DMY`, `MDY`, or `YMD`). These can be set separately or together. The keywords `Euro` and `European` are synonyms for `DMY`; the keywords `US`, `NonEuro`, and `NonEuropean` are synonyms for `MDY`. See [Date/Time Types](https://www.postgresql.org/docs/10/datatype-datetime.html) for more information. The built-in default is `ISO, MDY`, but initdb will initialize the configuration file with a setting that corresponds to the behavior of the chosen `lc_time` locale.
