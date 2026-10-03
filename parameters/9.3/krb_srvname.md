---
name: "krb_srvname"
version: "9.3"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Sets the name of the Kerberos service."
context: "sighup"
default: "postgres"
url: "https://www.postgresql.org/docs/9.3/runtime-config-connection.html#GUC-KRB-SRVNAME"
---

Sets the Kerberos service name. See [Kerberos Authentication](https://www.postgresql.org/docs/9.3/auth-methods.html#KERBEROS-AUTH) for details. This parameter can only be set in the `postgresql.conf` file or on the server command line.
