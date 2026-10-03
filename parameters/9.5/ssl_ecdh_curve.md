---
name: "ssl_ecdh_curve"
version: "9.5"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Sets the curve to use for ECDH."
context: "postmaster"
default: "prime256v1"
url: "https://www.postgresql.org/docs/9.5/runtime-config-connection.html#GUC-SSL-ECDH-CURVE"
---

Specifies the name of the curve to use in ECDH key exchange. It needs to be supported by all clients that connect. It does not need to be same curve as used by server's Elliptic Curve key. The default is `prime256v1`. This parameter can only be set at server start.

OpenSSL names for most common curves: `prime256v1` (NIST P-256), `secp384r1` (NIST P-384), `secp521r1` (NIST P-521).

The full list of available curves can be shown with the command `openssl ecparam -list_curves`. Not all of them are usable in TLS though.
