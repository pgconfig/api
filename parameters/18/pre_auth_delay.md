---
name: "pre_auth_delay"
version: "18"
type: "integer"
category: "Developer Options"
short_desc: "Sets the amount of time to wait before authentication on connection startup."
extra_desc: "This allows attaching a debugger to the process."
context: "sighup"
unit: "s"
default: "0"
min: "0"
max: "60"
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-PRE-AUTH-DELAY"
---

The amount of time to delay just after a new server process is forked, before it conducts the authentication procedure. This is intended to give developers an opportunity to attach to the server process with a debugger to trace down misbehavior in authentication. If this value is specified without units, it is taken as seconds. A value of zero (the default) disables the delay. This parameter can only be set in the `postgresql.conf` file or on the server command line.
