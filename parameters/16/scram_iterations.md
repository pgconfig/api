---
name: "scram_iterations"
version: "16"
type: "integer"
category: "Connections and Authentication / Authentication"
short_desc: "Sets the iteration count for SCRAM secret generation."
context: "user"
default: "4096"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/16/runtime-config-connection.html#GUC-SCRAM-ITERATIONS"
---

The number of computational iterations to be performed when encrypting a password using SCRAM-SHA-256. The default is `4096`. A higher number of iterations provides additional protection against brute-force attacks on stored passwords, but makes authentication slower. Changing the value has no effect on existing passwords encrypted with SCRAM-SHA-256 as the iteration count is fixed at the time of encryption. In order to make use of a changed value, a new password must be set.

> [!NOTE]
> If a role password was created with a different iteration count than the value of `scram_iterations` specified in the `postgresql.conf` file or on the server command line, an unauthenticated user can discern the existence of the role by observing discrepancies in the server's responses to connection attempts. If you find this concerning, ensure that all role passwords are created with `scram_iterations` set to the value specified in the `postgresql.conf` file or on the server command line.
