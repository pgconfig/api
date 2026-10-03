---
name: "gss_accept_delegation"
version: "18"
type: "boolean"
category: "Connections and Authentication / Authentication"
short_desc: "Sets whether GSSAPI delegation should be accepted from the client."
context: "sighup"
default: "off"
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-GSS-ACCEPT-DELEGATION"
---

Sets whether GSSAPI delegation should be accepted from the client. The default is `off` meaning credentials from the client will *not* be accepted. Changing this to `on` will make the server accept credentials delegated to it from the client. This parameter can only be set in the `postgresql.conf` file or on the server command line.
