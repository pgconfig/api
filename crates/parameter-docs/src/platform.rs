//! How a standard build of PostgreSQL for x86-64 Linux defines the macros
//! the GUC tables depend on: the configure defaults, `<limits.h>`, and the
//! choices the build makes for Linux.
//!
//! The extraction stops on a preprocessor condition this table does not
//! list, and on a value macro the headers define more than once, so that
//! every such decision is written down here.

/// A macro and its definition. `None` leaves it undefined.
const MACROS: &[(&str, Option<&str>)] = &[
    // <limits.h> and <float.h>.
    ("DBL_MAX", Some("1.7976931348623157e+308")),
    ("INT_MAX", Some("2147483647")),
    ("INT_MIN", Some("(-2147483647 - 1)")),
    ("IOV_MAX", Some("1024")),
    ("SIZE_MAX", Some("18446744073709551615")),
    // 64-bit: size_t and long have 8 bytes, so guc.h makes MAX_KILOBYTES
    // INT_MAX.
    ("SIZEOF_LONG", Some("8")),
    ("SIZEOF_SIZE_T", Some("8")),
    ("MAX_KILOBYTES", Some("INT_MAX")),
    // PostgreSQL 19's portability/instr_time.h enables the TSC clock on
    // x86-64. ARM builds accept only auto and system for timing_clock_source.
    ("PG_INSTR_TSC_CLOCK", Some("1")),
    // configure's defaults: 8kB pages, 16MB WAL segments, 1GB relation
    // segments, and the Kerberos settings under the default prefix.
    ("BLCKSZ", Some("8192")),
    ("XLOG_BLCKSZ", Some("8192")),
    ("XLOG_SEG_SIZE", Some("(16 * 1024 * 1024)")),
    ("RELSEG_SIZE", Some("131072")),
    ("DEF_PGPORT", Some("5432")),
    ("PG_KRB_SRVNAM", Some("\"postgres\"")),
    (
        "PG_KRB_SRVTAB",
        Some("\"FILE:/usr/local/pgsql/etc/krb5.keytab\""),
    ),
    // What Linux provides.
    ("HAVE_COPY_FILE_RANGE", Some("1")),
    ("HAVE_FDATASYNC", Some("1")),
    ("HAVE_POLL_H", Some("1")),
    ("HAVE_POSIX_FALLOCATE", Some("1")),
    ("HAVE_SYNCFS", Some("1")),
    ("HAVE_SYNC_FILE_RANGE", Some("1")),
    ("HAVE_SYSLOG", Some("1")),
    ("HAVE_UNIX_SOCKETS", Some("1")),
    ("O_DSYNC", Some("1")),
    ("O_SYNC", Some("1")),
    ("OPEN_DATASYNC_FLAG", Some("1")),
    ("OPEN_SYNC_FLAG", Some("1")),
    ("USE_PREFETCH", Some("1")),
    // POSIX shared memory is the default for dynamic shared memory, and
    // System V and mmap are available too.
    ("USE_DSM_MMAP", Some("1")),
    ("USE_DSM_POSIX", Some("1")),
    ("USE_DSM_SYSV", Some("1")),
    ("USE_DSM_WINDOWS", None),
    ("DEFAULT_DYNAMIC_SHARED_MEMORY_TYPE", Some("DSM_IMPL_POSIX")),
    // storage/fd.h defaults to the first method Linux has.
    (
        "DEFAULT_FILE_EXTEND_METHOD",
        Some("FILE_EXTEND_METHOD_POSIX_FALLOCATE"),
    ),
    // port/linux.h makes fdatasync the WAL default.
    ("DEFAULT_SYNC_METHOD", Some("SYNC_METHOD_FDATASYNC")),
    ("DEFAULT_WAL_SYNC_METHOD", Some("WAL_SYNC_METHOD_FDATASYNC")),
    // What only other systems have.
    ("COPYFILE_CLONE_FORCE", None),
    ("EXEC_BACKEND", None),
    ("HAVE_COPYFILE", None),
    ("HAVE_COPYFILE_H", None),
    ("HAVE_FSYNC_WRITETHROUGH", None),
    ("WIN32", None),
    // The libraries the PGDG packages and the official Docker images build
    // with, liburing included from PostgreSQL 18 on. Bonjour is for macOS.
    ("IOMETHOD_IO_URING_ENABLED", Some("1")),
    ("USE_LZ4", Some("1")),
    ("USE_OPENSSL", Some("1")),
    ("USE_SSL", Some("1")),
    ("USE_ZSTD", Some("1")),
    ("USE_BONJOUR", None),
    // Integer timestamps, the default from PostgreSQL 8.4 on.
    ("HAVE_INT64_TIMESTAMP", Some("1")),
    // pg_config_manual.h turns TRACE_SORT on, and leaves the other
    // debugging aids off. So does a build without assertions.
    ("TRACE_SORT", Some("1")),
    ("BTREE_BUILD_STATS", None),
    ("CLOBBER_CACHE_ALWAYS", None),
    ("CLOBBER_CACHE_RECURSIVELY", None),
    ("COPY_PARSE_PLAN_TREES", None),
    ("DEBUG_BOUNDED_SORT", None),
    ("DEBUG_NODE_TESTS_ENABLED", None),
    ("DISCARD_CACHES_ENABLED", None),
    ("LOCK_DEBUG", None),
    ("RAW_EXPRESSION_COVERAGE_TEST", None),
    ("TRACE_SYNCSCAN", None),
    ("USE_ASSERT_CHECKING", None),
    ("WAL_DEBUG", None),
    ("WRITE_READ_PARSE_PLAN_TREES", None),
];

/// `Some` when the table lists `name`, with its definition or `None`.
pub(crate) fn macro_value(name: &str) -> Option<Option<&'static str>> {
    MACROS
        .iter()
        .find(|(macro_name, _)| *macro_name == name)
        .map(|(_, value)| *value)
}
