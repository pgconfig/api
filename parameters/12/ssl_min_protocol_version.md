---
name: "ssl_min_protocol_version"
version: "12"
type: "enum"
category: "Connections and Authentication / SSL"
short_desc: "Sets the minimum SSL/TLS protocol version to use."
context: "sighup"
default: "TLSv1"
values: ["TLSv1", "TLSv1.1", "TLSv1.2", "TLSv1.3"]
url: "https://www.postgresql.org/docs/12/runtime-config-connection.html#GUC-SSL-MIN-PROTOCOL-VERSION"
---

Sets the minimum SSL/TLS protocol version to use. Valid values are currently: `TLSv1`, `TLSv1.1`, `TLSv1.2`, `TLSv1.3`. Older versions of the OpenSSL library do not support all values; an error will be raised if an unsupported setting is chosen. Protocol versions before TLS 1.0, namely SSL version 2 and 3, are always disabled.

The default is `TLSv1`, mainly to support older versions of the OpenSSL library. You might want to set this to a higher value if all software components can support the newer protocol versions.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
