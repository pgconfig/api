---
name: "zero_damaged_pages"
version: "9.1"
type: "boolean"
category: "Developer Options"
short_desc: "Continues processing past damaged page headers."
extra_desc: "Detection of a damaged page header normally causes PostgreSQL to report an error, aborting the current transaction. Setting zero_damaged_pages to true causes the system to instead report a warning, zero out the damaged page, and continue processing. This behavior will destroy data, namely all the rows on the damaged page."
context: "superuser"
default: "off"
url: "https://www.postgresql.org/docs/9.1/runtime-config-developer.html#GUC-ZERO-DAMAGED-PAGES"
---

Detection of a damaged page header normally causes PostgreSQL to report an error, aborting the current transaction. Setting `zero_damaged_pages` to on causes the system to instead report a warning, zero out the damaged page in memory, and continue processing. This behavior *will destroy data*, namely all the rows on the damaged page. However, it does allow you to get past the error and retrieve rows from any undamaged pages that might be present in the table. It is useful for recovering data if corruption has occurred due to a hardware or software error. You should generally not set this on until you have given up hope of recovering data from the damaged pages of a table. Zeroed-out pages are not forced to disk so it is recommended to recreate the table or the index before turning this parameter off again. The default setting is `off`, and it can only be changed by a superuser.
