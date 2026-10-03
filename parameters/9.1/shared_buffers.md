---
name: "shared_buffers"
version: "9.1"
type: "integer"
category: "Resource Usage / Memory"
short_desc: "Sets the number of shared memory buffers used by the server."
context: "postmaster"
unit: "8kB"
default: "1024"
min: "16"
max: "1073741823"
url: "https://www.postgresql.org/docs/9.1/runtime-config-resource.html#GUC-SHARED-BUFFERS"
---

Sets the amount of memory the database server uses for shared memory buffers. The default is typically 32 megabytes (`32MB`), but might be less if your kernel settings will not support it (as determined during initdb). This setting must be at least 128 kilobytes. (Non-default values of `BLCKSZ` change the minimum.) However, settings significantly higher than the minimum are usually needed for good performance. This parameter can only be set at server start.

If you have a dedicated database server with 1GB or more of RAM, a reasonable starting value for `shared_buffers` is 25% of the memory in your system. There are some workloads where even large settings for `shared_buffers` are effective, but because PostgreSQL also relies on the operating system cache, it is unlikely that an allocation of more than 40% of RAM to `shared_buffers` will work better than a smaller amount. Larger settings for `shared_buffers` usually require a corresponding increase in `checkpoint_segments`, in order to spread out the process of writing large quantities of new or changed data over a longer period of time.

On systems with less than 1GB of RAM, a smaller percentage of RAM is appropriate, so as to leave adequate space for the operating system. Also, on Windows, large values for `shared_buffers` aren't as effective. You may find better results keeping the setting relatively low and using the operating system cache more instead. The useful range for `shared_buffers` on Windows systems is generally from 64MB to 512MB.

Increasing this parameter might cause PostgreSQL to request more `System V` shared memory than your operating system's default configuration allows. See [Shared Memory and Semaphores](https://www.postgresql.org/docs/9.1/kernel-resources.html#SYSVIPC) for information on how to adjust those parameters, if necessary.
