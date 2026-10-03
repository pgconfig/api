---
name: "krb_server_keyfile"
version: "18"
type: "string"
category: "Connections and Authentication / Authentication"
short_desc: "Sets the location of the Kerberos server key file."
context: "sighup"
default: "FILE:/usr/local/pgsql/etc/krb5.keytab"
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-KRB-SERVER-KEYFILE"
---

Sets the location of the server's Kerberos key file. The default is `FILE:/usr/local/pgsql/etc/krb5.keytab` (where the directory part is whatever was specified as `sysconfdir` at build time; use `pg_config --sysconfdir` to determine that). If this parameter is set to an empty string, it is ignored and a system-dependent default is used. This parameter can only be set in the `postgresql.conf` file or on the server command line. See [GSSAPI Authentication](https://www.postgresql.org/docs/18/gssapi-auth.html) for more information.
