---
name: "allow_alter_system"
version: "18"
type: "boolean"
category: "Version and Platform Compatibility / Other Platforms and Clients"
short_desc: "Allows running the ALTER SYSTEM command."
extra_desc: "Can be set to off for environments where global configuration changes should be made using a different method."
context: "sighup"
default: "on"
url: "https://www.postgresql.org/docs/18/runtime-config-compatible.html#GUC-ALLOW-ALTER-SYSTEM"
---

When `allow_alter_system` is set to `off`, an error is returned if the `ALTER SYSTEM` command is executed. This parameter can only be set in the `postgresql.conf` file or on the server command line. The default value is `on`.

Note that this setting must not be regarded as a security feature. It only disables the `ALTER SYSTEM` command. It does not prevent a superuser from changing the configuration using other SQL commands. A superuser has many ways of executing shell commands at the operating system level, and can therefore modify `postgresql.auto.conf` regardless of the value of this setting.

Turning this setting off is intended for environments where the configuration of PostgreSQL is managed by some external tool. In such environments, a well-intentioned superuser might *mistakenly* use `ALTER SYSTEM` to change the configuration instead of using the external tool. This might result in unintended behavior, such as the external tool overwriting the change at some later point in time when it updates the configuration. Setting this parameter to `off` can help avoid such mistakes.

This parameter only controls the use of `ALTER SYSTEM`. The settings stored in `postgresql.auto.conf` take effect even if `allow_alter_system` is set to `off`.
