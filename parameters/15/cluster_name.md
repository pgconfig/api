---
name: "cluster_name"
version: "15"
type: "string"
category: "Reporting and Logging / Process Title"
short_desc: "Sets the name of the cluster, which is included in the process title."
context: "postmaster"
default: ""
url: "https://www.postgresql.org/docs/15/runtime-config-logging.html#GUC-CLUSTER-NAME"
---

Sets a name that identifies this database cluster (instance) for various purposes. The cluster name appears in the process title for all server processes in this cluster. Moreover, it is the default application name for a standby connection (see [`synchronous_standby_names`](https://www.postgresql.org/docs/15/runtime-config-replication.html#GUC-SYNCHRONOUS-STANDBY-NAMES)).

The name can be any string of less than `NAMEDATALEN` characters (64 characters in a standard build). Only printable ASCII characters may be used in the `cluster_name` value. Other characters will be replaced with question marks (`?`). No name is shown if this parameter is set to the empty string `''` (which is the default). This parameter can only be set at server start.
