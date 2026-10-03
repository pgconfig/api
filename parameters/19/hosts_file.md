---
name: "hosts_file"
version: "19"
type: "string"
category: "File Locations"
short_desc: "Sets the server's \"hosts\" configuration file."
context: "postmaster"
url: "https://www.postgresql.org/docs/19/runtime-config-file-locations.html#GUC-HOSTS-FILE"
---

Specifies the configuration file for host-based SSL configuration (customarily called `pg_hosts.conf`). This parameter can only be set at server start. See also [SNI Configuration](https://www.postgresql.org/docs/19/ssl-tcp.html#SSL-SNI).
