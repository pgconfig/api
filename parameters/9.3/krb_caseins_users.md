---
name: "krb_caseins_users"
version: "9.3"
type: "boolean"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Sets whether Kerberos and GSSAPI user names should be treated as case-insensitive."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/9.3/runtime-config-connection.html#GUC-KRB-CASEINS-USERS"
---

Sets whether Kerberos and GSSAPI user names should be treated case-insensitively. The default is `off` (case sensitive). This parameter can only be set in the `postgresql.conf` file or on the server command line.
