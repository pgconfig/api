---
name: "pre_auth_delay"
version: "11"
type: "integer"
category: "Developer Options"
short_desc: "Waits N seconds on connection startup before authentication."
extra_desc: "This allows attaching a debugger to the process."
context: "sighup"
unit: "s"
default: "0"
min: "0"
max: "60"
url: "https://www.postgresql.org/docs/11/runtime-config-developer.html#GUC-PRE-AUTH-DELAY"
---

If nonzero, a delay of this many seconds occurs just after a new server process is forked, before it conducts the authentication procedure. This is intended to give developers an opportunity to attach to the server process with a debugger to trace down misbehavior in authentication. This parameter can only be set in the `postgresql.conf` file or on the server command line.
