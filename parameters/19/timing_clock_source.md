---
name: "timing_clock_source"
version: "19"
type: "enum"
category: "Resource Usage / Time"
short_desc: "Controls the clock source used for collecting timing measurements."
extra_desc: "This enables the use of specialized clock sources, specifically the RDTSC clock source on x86-64 systems (if available), to support timing measurements with lower overhead during EXPLAIN and other instrumentation."
context: "superuser"
default: "auto"
values: ["auto", "system", "tsc"]
url: "https://www.postgresql.org/docs/19/runtime-config-resource.html#GUC-TIMING-CLOCK-SOURCE"
---

Selects the method for making timing measurements using the OS or specialized CPU instructions. Possible values are:

- `auto` (automatically chooses TSC clock source on supported x86-64 CPUs, otherwise uses the OS system clock)
- `system` (measures timing using the OS system clock)
- `tsc` (measures timing with a CPU instruction, e.g. using `RDTSC`/`RDTSCP` on x86-64)

The default is `auto`. Only superusers can change this setting. Changing the setting during query execution is not recommended and may cause interval timings to jump significantly or produce negative values.

If enabled, the TSC clock source, named after the Time-Stamp Counter on x86-64, will use specialized CPU instructions when measuring time intervals. This lowers timing overhead compared to reading the OS system clock, and reduces the measurement error on top of the actual runtime, for example with `EXPLAIN ANALYZE`.

On x86-64 CPUs the TSC clock source utilizes the `RDTSC` instruction for `EXPLAIN ANALYZE`. For timings that require higher precision the `RDTSCP` instruction is used, which avoids inaccuracies due to CPU instruction re-ordering. Use of the TSC clock source is not supported on older x86-64 CPUs and other architectures, and is not advised on systems that utilize an emulated TSC, as it is likely slower than the system clock source.

To help decide which clock source to use you can run the [pg_test_timing](https://www.postgresql.org/docs/19/pgtesttiming.html) utility to check TSC availability, and perform timing measurements.
