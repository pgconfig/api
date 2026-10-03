---
name: "ssl_tls13_ciphers"
version: "19"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Sets the list of allowed TLSv1.3 cipher suites."
extra_desc: "An empty string means use the default cipher suites."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-SSL-TLS13-CIPHERS"
---

Specifies a list of cipher suites that are allowed by connections using TLS version 1.3. Multiple cipher suites can be specified by using a colon-separated list. If left blank, the default set of cipher suites in OpenSSL will be used.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
