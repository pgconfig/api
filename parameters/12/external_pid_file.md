---
name: "external_pid_file"
version: "12"
type: "string"
category: "File Locations"
short_desc: "Writes the postmaster PID to the specified file."
context: "postmaster"
url: "https://www.postgresql.org/docs/12/runtime-config-file-locations.html#GUC-EXTERNAL-PID-FILE"
---

Specifies the name of an additional process-ID (PID) file that the server should create for use by server administration programs. This parameter can only be set at server start.
