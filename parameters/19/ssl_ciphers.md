---
name: "ssl_ciphers"
version: "19"
type: "string"
category: "Connections and Authentication / SSL"
short_desc: "Sets the list of allowed TLSv1.2 (and lower) ciphers."
context: "sighup"
default: "HIGH:MEDIUM:+3DES:!aNULL"
url: "https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-SSL-CIPHERS"
---

Specifies a list of SSL ciphers that are allowed by connections using TLS version 1.2 and lower, see [`ssl_tls13_ciphers`](https://www.postgresql.org/docs/19/runtime-config-connection.html#GUC-SSL-TLS13-CIPHERS) for TLS version 1.3 connections. See the ciphers() manual page in the OpenSSL package for the syntax of this setting and a list of supported values. The default value is `HIGH:MEDIUM:+3DES:!aNULL`. The default is usually a reasonable choice unless you have specific security requirements.

This parameter can only be set in the `postgresql.conf` file or on the server command line.

Explanation of the default value:

- `HIGH`: Cipher suites that use ciphers from `HIGH` group (e.g., AES, Camellia, 3DES)
- `MEDIUM`: Cipher suites that use ciphers from `MEDIUM` group (e.g., RC4, SEED)
- `+3DES`: The OpenSSL default order for `HIGH` is problematic because it orders 3DES higher than AES128. This is wrong because 3DES offers less security than AES128, and it is also much slower. `+3DES` reorders it after all other `HIGH` and `MEDIUM` ciphers.
- `!aNULL`: Disables anonymous cipher suites that do no authentication. Such cipher suites are vulnerable to MITM attacks and therefore should not be used.

Available cipher suite details will vary across OpenSSL versions. Use the command `openssl ciphers -v 'HIGH:MEDIUM:+3DES:!aNULL'` to see actual details for the currently installed OpenSSL version. Note that this list is filtered at run time based on the server key type.
