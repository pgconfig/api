---
name: "io_method"
version: "19"
type: "enum"
category: "Resource Usage / I/O"
short_desc: "Selects the method for executing asynchronous I/O."
context: "postmaster"
default: "worker"
values: ["sync", "worker", "io_uring"]
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-IO-METHOD"
---

Selects the method for executing asynchronous I/O. Possible values are:

- `worker` (execute asynchronous I/O using worker processes)
- `io_uring` (execute asynchronous I/O using io_uring, requires a build with [`--with-liburing`](https://www.postgresql.org/docs/19/install-make.html#CONFIGURE-OPTION-WITH-LIBURING) / [`-Dliburing`](https://www.postgresql.org/docs/19/install-meson.html#CONFIGURE-WITH-LIBURING-MESON))
- `sync` (execute asynchronous-eligible I/O synchronously)

The default is `worker`.

This parameter can only be set at server start.
