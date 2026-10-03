---
name: "event_triggers"
version: "17"
type: "boolean"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Enables event triggers."
extra_desc: "When enabled, event triggers will fire for all applicable statements."
context: "superuser"
default: "on"
url: "https://www.postgresql.org/docs/17/runtime-config-client.html#GUC-EVENT-TRIGGERS"
---

Allow temporarily disabling execution of event triggers in order to troubleshoot and repair faulty event triggers. All event triggers will be disabled by setting it to `false`. Setting the value to `true` allows all event triggers to fire, this is the default value. Only superusers and users with the appropriate `SET` privilege can change this setting.
