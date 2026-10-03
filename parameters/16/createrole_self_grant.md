---
name: "createrole_self_grant"
version: "16"
type: "string"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets whether a CREATEROLE user automatically grants the role to themselves, and with which options."
context: "user"
default: ""
url: "https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-CREATEROLE-SELF-GRANT"
---

If a user who has `CREATEROLE` but not `SUPERUSER` creates a role, and if this is set to a non-empty value, the newly-created role will be granted to the creating user with the options specified. The value must be `set`, `inherit`, or a comma-separated list of these. The default value is an empty string, which disables the feature.

The purpose of this option is to allow a `CREATEROLE` user who is not a superuser to automatically inherit, or automatically gain the ability to `SET ROLE` to, any created users. Since a `CREATEROLE` user is always implicitly granted `ADMIN OPTION` on created roles, that user could always execute a `GRANT` statement that would achieve the same effect as this setting. However, it can be convenient for usability reasons if the grant happens automatically. A superuser automatically inherits the privileges of every role and can always `SET ROLE` to any role, and this setting can be used to produce a similar behavior for `CREATEROLE` users for users which they create.
