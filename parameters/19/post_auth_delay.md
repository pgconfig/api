---
name: "post_auth_delay"
version: "19"
type: "integer"
category: "Developer Options"
short_desc: "Sets the amount of time to wait after authentication on connection startup."
extra_desc: "This allows attaching a debugger to the process."
context: "backend"
unit: "s"
default: "0"
min: "0"
max: "2147"
url: "https://www.postgresql.org/docs/19/runtime-config-developer.html#GUC-POST-AUTH-DELAY"
---

The amount of time to delay when a new server process is started, after it conducts the authentication procedure. This is intended to give developers an opportunity to attach to the server process with a debugger. If this value is specified without units, it is taken as seconds. A value of zero (the default) disables the delay. This parameter cannot be changed after session start.
