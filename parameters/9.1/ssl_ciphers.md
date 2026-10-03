---
name: "ssl_ciphers"
version: "9.1"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Sets the list of allowed SSL ciphers."
context: "postmaster"
default: "ALL:!ADH:!LOW:!EXP:!MD5:@STRENGTH"
url: "https://www.postgresql.org/docs/9.1/runtime-config-connection.html#GUC-SSL-CIPHERS"
---

Specifies a list of SSL ciphers that are allowed to be used on secure connections. See the openssl manual page for a list of supported ciphers.
