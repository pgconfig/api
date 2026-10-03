---
name: "cluster_name"
version: "11"
type: "string"
category: "Process Title"
short_desc: "Sets the name of the cluster, which is included in the process title."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/11/runtime-config-logging.html#GUC-CLUSTER-NAME"
---

Sets the cluster name that appears in the process title for all server processes in this cluster. The name can be any string of less than `NAMEDATALEN` characters (64 characters in a standard build). Only printable ASCII characters may be used in the `cluster_name` value. Other characters will be replaced with question marks (`?`). No name is shown if this parameter is set to the empty string `''` (which is the default). This parameter can only be set at server start.
