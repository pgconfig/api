---
name: "default_text_search_config"
version: "9.5"
type: "string"
category: "Client Connection Defaults / Locale and Formatting"
short_desc: "Sets default text search configuration."
context: "user"
default: "pg_catalog.simple"
url: "https://www.postgresql.org/docs/9.5/runtime-config-client.html#GUC-DEFAULT-TEXT-SEARCH-CONFIG"
---

Selects the text search configuration that is used by those variants of the text search functions that do not have an explicit argument specifying the configuration. See [Full Text Search](https://www.postgresql.org/docs/9.5/textsearch.html) for further information. The built-in default is `pg_catalog.simple`, but initdb will initialize the configuration file with a setting that corresponds to the chosen `lc_ctype` locale, if a configuration matching that locale can be identified.
