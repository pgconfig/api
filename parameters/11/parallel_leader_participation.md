---
name: "parallel_leader_participation"
version: "11"
type: "boolean"
category: "Resource Usage / Asynchronous Behavior"
short_desc: "Controls whether Gather and Gather Merge also run subplans."
extra_desc: "Should gather nodes also run subplans, or just gather tuples?"
context: "user"
default: "on"
url: "https://www.postgresql.org/docs/11/runtime-config-query.html#GUC-PARALLEL-LEADER-PARTICIPATION"
---

Allows the leader process to execute the query plan under `Gather` and `Gather Merge` nodes instead of waiting for worker processes. The default is `on`. Setting this value to `off` reduces the likelihood that workers will become blocked because the leader is not reading tuples fast enough, but requires the leader process to wait for worker processes to start up before the first tuples can be produced. The degree to which the leader can help or hinder performance depends on the plan type, number of workers and query duration.
