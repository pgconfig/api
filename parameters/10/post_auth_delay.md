---
name: "post_auth_delay"
version: "10"
type: "integer"
category: "Developer Options"
short_desc: "Waits N seconds on connection startup after authentication."
extra_desc: "This allows attaching a debugger to the process."
context: "backend"
unit: "s"
default: "0"
min: "0"
max: "2147"
url: "https://www.postgresql.org/docs/10/runtime-config-developer.html#GUC-POST-AUTH-DELAY"
---

If nonzero, a delay of this many seconds occurs when a new server process is started, after it conducts the authentication procedure. This is intended to give developers an opportunity to attach to the server process with a debugger. This parameter cannot be changed after session start.
