---
name: "restrict_nonsystem_relation_kind"
version: "14"
type: "string"
category: "Client Connection Defaults / Statement Behavior"
short_desc: "Prohibits access to non-system relations of specified kinds."
context: "user"
default: ""
url: "https://www.postgresql.org/docs/14/runtime-config-client.html#GUC-RESTRICT-NONSYSTEM-RELATION-KIND"
---

Set relation kinds for which access to non-system relations is prohibited. The value takes the form of a comma-separated list of relation kinds. Currently, the supported relation kinds are `view` and `foreign-table`.
