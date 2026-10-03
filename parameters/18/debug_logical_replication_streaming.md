---
name: "debug_logical_replication_streaming"
version: "18"
type: "enum"
category: "Developer Options"
short_desc: "Forces immediate streaming or serialization of changes in large transactions."
extra_desc: "On the publisher, it allows streaming or serializing each change in logical decoding. On the subscriber, it allows serialization of all changes to files and notifies the parallel apply workers to read and apply them at the end of the transaction."
context: "user"
default: "buffered"
values: ["buffered", "immediate"]
url: "https://www.postgresql.org/docs/18/runtime-config-developer.html#GUC-DEBUG-LOGICAL-REPLICATION-STREAMING"
---

The allowed values are `buffered` and `immediate`. The default is `buffered`. This parameter is intended to be used to test logical decoding and replication of large transactions. The effect of `debug_logical_replication_streaming` is different for the publisher and subscriber:

On the publisher side, `debug_logical_replication_streaming` allows streaming or serializing changes immediately in logical decoding. When set to `immediate`, stream each change if the [`streaming`](https://www.postgresql.org/docs/18/sql-createsubscription.html#SQL-CREATESUBSCRIPTION-PARAMS-WITH-STREAMING) option of [`CREATE SUBSCRIPTION`](https://www.postgresql.org/docs/18/sql-createsubscription.html) is enabled, otherwise, serialize each change. When set to `buffered`, the decoding will stream or serialize changes when `logical_decoding_work_mem` is reached.

On the subscriber side, if the `streaming` option is set to `parallel`, `debug_logical_replication_streaming` can be used to direct the leader apply worker to send changes to the shared memory queue or to serialize all changes to the file. When set to `buffered`, the leader sends changes to parallel apply workers via a shared memory queue. When set to `immediate`, the leader serializes all changes to files and notifies the parallel apply workers to read and apply them at the end of the transaction.
