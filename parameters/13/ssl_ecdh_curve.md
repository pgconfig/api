---
name: "ssl_ecdh_curve"
version: "13"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Sets the curve to use for ECDH."
context: "sighup"
default: "prime256v1"
url: "https://www.postgresql.org/docs/13/runtime-config-connection.html#GUC-SSL-ECDH-CURVE"
---

Specifies the name of the curve to use in ECDH key exchange. It needs to be supported by all clients that connect. It does not need to be the same curve used by the server's Elliptic Curve key. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default is `prime256v1`.

OpenSSL names for the most common curves are: `prime256v1` (NIST P-256), `secp384r1` (NIST P-384), `secp521r1` (NIST P-521). The full list of available curves can be shown with the command `openssl ecparam -list_curves`. Not all of them are usable in TLS though.
