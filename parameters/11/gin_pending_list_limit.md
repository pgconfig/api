---
name: "gin_pending_list_limit"
version: "11"
type: "integer"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Sets the maximum size of the pending list for GIN index."
context: "user"
unit: "kB"
default: "4096"
min: "64"
max: "2147483647"
url: "https://www.postgresql.org/docs/11/runtime-config-client.html#GUC-GIN-PENDING-LIST-LIMIT"
---

Sets the maximum size of the GIN pending list which is used when `fastupdate` is enabled. If the list grows larger than this maximum size, it is cleaned up by moving the entries in it to the main GIN data structure in bulk. The default is four megabytes (`4MB`). This setting can be overridden for individual GIN indexes by changing index storage parameters. See [GIN Fast Update Technique](https://www.postgresql.org/docs/11/gin-implementation.html#GIN-FAST-UPDATE) and [GIN Tips and Tricks](https://www.postgresql.org/docs/11/gin-tips.html) for more information.
