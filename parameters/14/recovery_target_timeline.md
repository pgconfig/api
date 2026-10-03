---
name: "recovery_target_timeline"
version: "14"
type: "string"
category: "Write-Ahead Log / Recovery Target"
short_desc: "Specifies the timeline to recover into."
context: "postmaster"
default: "latest"
url: "https://www.postgresql.org/docs/14/runtime-config-wal.html#GUC-RECOVERY-TARGET-TIMELINE"
---

Specifies recovering into a particular timeline. The value can be a numeric timeline ID or a special value. The value `current` recovers along the same timeline that was current when the base backup was taken. The value `latest` recovers to the latest timeline found in the archive, which is useful in a standby server. `latest` is the default.

You usually only need to set this parameter in complex re-recovery situations, where you need to return to a state that itself was reached after a point-in-time recovery. See [Timelines](https://www.postgresql.org/docs/14/continuous-archiving.html#BACKUP-TIMELINES) for discussion.
