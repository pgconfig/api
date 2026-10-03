---
name: "max_files_per_process"
version: "9.2"
type: "integer"
category: "Resource Usage / Kernel Resources"
short_desc: "Sets the maximum number of simultaneously open files for each server process."
context: "postmaster"
default: "1000"
min: "25"
max: "2147483647"
url: "https://www.postgresql.org/docs/9.2/runtime-config-resource.html#GUC-MAX-FILES-PER-PROCESS"
---

Sets the maximum number of simultaneously open files allowed to each server subprocess. The default is one thousand files. If the kernel is enforcing a safe per-process limit, you don't need to worry about this setting. But on some platforms (notably, most BSD systems), the kernel will allow individual processes to open many more files than the system can actually support if many processes all try to open that many files. If you find yourself seeing "Too many open files" failures, try reducing this setting. This parameter can only be set at server start.
