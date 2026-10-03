---
name: "deadlock_timeout"
version: "9.2"
type: "integer"
category: "Lock Management"
short_desc: "Sets the time to wait on a lock before checking for deadlock."
context: "superuser"
unit: "ms"
default: "1000"
min: "1"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.2/runtime-config-locks.html#GUC-DEADLOCK-TIMEOUT"
---

This is the amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition. The check for deadlock is relatively expensive, so the server doesn't run it every time it waits for a lock. We optimistically assume that deadlocks are not common in production applications and just wait on the lock for a while before checking for a deadlock. Increasing this value reduces the amount of time wasted in needless deadlock checks, but slows down reporting of real deadlock errors. The default is one second (`1s`), which is probably about the smallest value you would want in practice. On a heavily loaded server you might want to raise it. Ideally the setting should exceed your typical transaction time, so as to improve the odds that a lock will be released before the waiter decides to check for deadlock. Only superusers can change this setting.

When [`log_lock_waits`](https://www.postgresql.org/docs/9.2/runtime-config-logging.html#GUC-LOG-LOCK-WAITS) is set, this parameter also determines the length of time to wait before a log message is issued about the lock wait. If you are trying to investigate locking delays you might want to set a shorter than normal `deadlock_timeout`.
