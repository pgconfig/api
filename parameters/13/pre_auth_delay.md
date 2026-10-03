---
name: "pre_auth_delay"
version: "13"
type: "integer"
category: "Developer Options"
short_desc: "Waits N seconds on connection startup before authentication."
extra_desc: "This allows attaching a debugger to the process."
context: "sighup"
unit: "s"
default: "0"
min: "0"
max: "60"
url: "https://www.postgresql.org/docs/13/runtime-config-developer.html#GUC-PRE-AUTH-DELAY"
---

The amount of time to delay just after a new server process is forked, before it conducts the authentication procedure. This is intended to give developers an opportunity to attach to the server process with a debugger to trace down misbehavior in authentication. If this value is specified without units, it is taken as seconds. A value of zero (the default) disables the delay. This parameter can only be set in the `postgresql.conf` file or on the server command line.
