---
name: "ssl_dh_params_file"
version: "10"
type: "string"
category: "Connections and Authentication / Security and Authentication"
short_desc: "Location of the SSL DH parameters file."
context: "sighup"
default: ""
url: "https://www.postgresql.org/docs/10/runtime-config-connection.html#GUC-SSL-DH-PARAMS-FILE"
---

Specifies the name of the file containing Diffie-Hellman parameters used for so-called ephemeral DH family of SSL ciphers. The default is empty, in which case compiled-in default DH parameters used. Using custom DH parameters reduces the exposure if an attacker manages to crack the well-known compiled-in DH parameters. You can create your own DH parameters file with the command `openssl dhparam -out dhparams.pem 2048`.

This parameter can only be set in the `postgresql.conf` file or on the server command line.
