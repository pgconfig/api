---
name: "ssl_groups"
version: "18"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Sets the group(s) to use for Diffie-Hellman key exchange."
extra_desc: "Multiple groups can be specified using a colon-separated list."
context: "sighup"
default: "X25519:prime256v1"
url: "https://www.postgresql.org/docs/18/runtime-config-connection.html#GUC-SSL-GROUPS"
---

Specifies the named group to use for TLS key exchange. It needs to be supported by all clients that connect. Multiple groups can be specified by using a colon-separated list. It does not need to match the key type used by the server certificate. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `X25519:prime256v1`.

OpenSSL names for the most common groups are: `prime256v1` (NIST P-256), `secp384r1` (NIST P-384), `secp521r1` (NIST P-521). An incomplete list of available groups can be shown with the command `openssl ecparam -list_curves`. Not all of them are usable with TLS though, and many supported group names and aliases are omitted.

In PostgreSQL versions before 18.0 this setting was named `ssl_ecdh_curve` and only accepted a single value.
