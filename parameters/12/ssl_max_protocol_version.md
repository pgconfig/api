---
name: "ssl_max_protocol_version"
version: "12"
type: "enum"
category: "Connections and Authentication / SSL"
short_desc: "Sets the maximum SSL/TLS protocol version to use."
context: "sighup"
default: ""
values: ["", "TLSv1", "TLSv1.1", "TLSv1.2", "TLSv1.3"]
url: "https://www.postgresql.org/docs/12/runtime-config-connection.html#GUC-SSL-MAX-PROTOCOL-VERSION"
---

Sets the maximum SSL/TLS protocol version to use. Valid values are as for [`ssl_min_protocol_version`](https://www.postgresql.org/docs/12/runtime-config-connection.html#GUC-SSL-MIN-PROTOCOL-VERSION), with addition of an empty string, which allows any protocol version. The default is to allow any version. Setting the maximum protocol version is mainly useful for testing or if some component has issues working with a newer protocol.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
