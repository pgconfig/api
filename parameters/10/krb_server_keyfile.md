---
name: "krb_server_keyfile"
version: "10"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Sets the location of the Kerberos server key file."
context: "sighup"
default: "FILE:/usr/local/pgsql/etc/krb5.keytab"
url: "https://www.postgresql.org/docs/10/runtime-config-connection.html#GUC-KRB-SERVER-KEYFILE"
---

Sets the location of the Kerberos server key file. See [GSSAPI Authentication](https://www.postgresql.org/docs/10/auth-methods.html#GSSAPI-AUTH) for details. This parameter can only be set in the `postgresql.conf` file or on the server command line.
