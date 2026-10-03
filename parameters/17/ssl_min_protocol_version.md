---
name: "ssl_min_protocol_version"
version: "17"
type: "enum"
category: "Connections and Authentication / SSL"
short_desc: "Sets the minimum SSL/TLS protocol version to use."
context: "sighup"
default: "TLSv1.2"
values: ["TLSv1", "TLSv1.1", "TLSv1.2", "TLSv1.3"]
url: "https://www.postgresql.org/docs/17/runtime-config-connection.html#GUC-SSL-MIN-PROTOCOL-VERSION"
---

Sets the minimum SSL/TLS protocol version to use. Valid values are currently: `TLSv1`, `TLSv1.1`, `TLSv1.2`, `TLSv1.3`. Older versions of the OpenSSL library do not support all values; an error will be raised if an unsupported setting is chosen. Protocol versions before TLS 1.0, namely SSL version 2 and 3, are always disabled.

The default is `TLSv1.2`, which satisfies industry best practices as of this writing.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
