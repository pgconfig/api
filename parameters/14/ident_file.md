---
name: "ident_file"
version: "14"
type: "string"
category: "File Locations"
short_desc: "Sets the server's \"ident\" configuration file."
context: "postmaster"
url: "https://www.postgresql.org/docs/14/runtime-config-file-locations.html#GUC-IDENT-FILE"
---

Specifies the configuration file for user name mapping (customarily called `pg_ident.conf`). This parameter can only be set at server start. See also [User Name Maps](https://www.postgresql.org/docs/14/auth-username-maps.html).
