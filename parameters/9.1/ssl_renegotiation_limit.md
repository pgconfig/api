---
name: "ssl_renegotiation_limit"
version: "9.1"
type: "integer"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Set the amount of traffic to send and receive before renegotiating the encryption keys."
context: "user"
unit: "kB"
default: "0"
min: "0"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.1/runtime-config-connection.html#GUC-SSL-RENEGOTIATION-LIMIT"
---

Specifies how much data can flow over an SSL-encrypted connection before renegotiation of the session keys will take place. Renegotiation decreases an attacker's chances of doing cryptanalysis when large amounts of traffic can be examined, but it also carries a large performance penalty. The sum of sent and received traffic is used to check the limit. If this parameter is set to 0, renegotiation is disabled. The default is `0`.

> [!NOTE]
> SSL libraries from before November 2009 are insecure when using SSL renegotiation, due to a vulnerability in the SSL protocol. As a stop-gap fix for this vulnerability, some vendors shipped SSL libraries incapable of doing renegotiation. If any such libraries are in use on the client or server, SSL renegotiation should be disabled.

> [!WARNING]
> Due to bugs in OpenSSL enabling ssl renegotiation, by configuring a non-zero `ssl_renegotiation_limit`, is likely to lead to problems like long-lived connections breaking.
