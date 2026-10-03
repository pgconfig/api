//! The tuning rules. This is the one place values are computed: the rich
//! result and the v1 projection both start from `compute`.
//!
//! The arithmetic follows the Go engine operation by operation, in `f32`
//! where Go used `float32`, because the outputs are pinned by the goldens in
//! `tests/golden`.

use crate::bytes::{GB, KB, MB};
use crate::request::Profile;

/// The largest `work_mem` and `maintenance_work_mem` on Windows before
/// PostgreSQL 18. Those releases sized `MAX_KILOBYTES` by `sizeof(long)`, which
/// is 4 on 64-bit Windows.
/// <https://www.postgresql.org/message-id/flat/1a01f0-66ec2d80-3b-68487680@27595217>
const WINDOWS_MAX_WORK_MEM: i64 = 2_097_151 * KB;

/// PostgreSQL rejects `io_workers` above 32.
const MAX_IO_WORKERS: i64 = 32;

/// The storage under the data directory, as the rules see it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Disk {
    Hdd,
    Ssd,
    San,
    /// A drive type REST v1 did not recognize. v1 never rejected one: it gets
    /// the HDD `effective_io_concurrency` and the SSD `random_page_cost`.
    Unrecognized,
}

/// Everything the rules read. Callers validate; nothing here does.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Facts {
    pub total_ram: i64,
    pub total_cpu: i64,
    pub max_connections: i64,
    pub profile: Profile,
    pub disk: Disk,
    /// Whether the Windows rules apply.
    pub windows: bool,
    /// Whether PostgreSQL is a 32-bit build.
    pub thirty_two_bit: bool,
    /// The release on the v1 float scale: 9.6 for PostgreSQL 9.6, 18.0 for 18.
    pub version: f32,
}

/// A limit that lowered a memory setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Limit {
    /// 4GB on a 32-bit build.
    ThirtyTwoBit,
    /// `shared_buffers` of 512MB up to PostgreSQL 9.6.
    Through96,
    /// `shared_buffers` of 8GB before PostgreSQL 9.6, where larger values
    /// slowed the server down.
    Before96,
    /// The Windows limit before PostgreSQL 18.
    WindowsBefore18,
}

/// A memory setting and the limits that lowered it, in the order they applied.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Memory {
    pub value: i64,
    /// Each limit with the value it replaced.
    pub limits: Vec<(Limit, i64)>,
}

impl Memory {
    fn new(value: i64) -> Self {
        Self {
            value,
            limits: Vec::new(),
        }
    }

    fn limit(&mut self, limit: Limit, max: i64) {
        if self.value > max {
            self.limits.push((limit, self.value));
            self.value = max;
        }
    }
}

/// How `io_workers` was reached.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct IoWorkers {
    pub value: i64,
    /// The value before the bounds below.
    pub initial: i64,
    pub hdd_adjusted: bool,
    pub raised_to_minimum: bool,
    pub capped_at_cpus: bool,
    pub capped_at_maximum: bool,
}

/// The asynchronous I/O settings of PostgreSQL 18 and later.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Aio {
    pub io_method: &'static str,
    pub io_workers: Option<IoWorkers>,
    pub io_min_workers: Option<i64>,
    pub io_max_workers: Option<i64>,
    pub io_max_combine_limit: i64,
    pub io_max_concurrency: i64,
    pub file_copy_method: &'static str,
}

/// Every setting the rules produce. A setting the PostgreSQL release does not
/// have is `None`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Computed {
    pub shared_buffers: Memory,
    /// Whether the DESKTOP profile replaced `shared_buffers` with its own
    /// share of RAM, discarding the limits applied before it.
    pub desktop_shared_buffers: bool,
    pub effective_cache_size: i64,
    pub work_mem: Memory,
    pub maintenance_work_mem: Memory,

    pub min_wal_size: Option<i64>,
    pub max_wal_size: Option<i64>,
    pub checkpoint_completion_target: f32,
    pub wal_buffers: i64,
    pub checkpoint_segments: Option<i64>,

    pub listen_addresses: &'static str,
    pub max_connections: i64,

    pub random_page_cost: f32,
    pub effective_io_concurrency: i64,
    pub maintenance_io_concurrency: Option<i64>,
    pub aio: Option<Aio>,

    pub max_worker_processes: Option<i64>,
    pub max_parallel_workers_per_gather: Option<i64>,
    pub max_parallel_workers: Option<i64>,
}

/// The share of RAM a profile may use at all.
fn profile_memory_share(profile: Profile) -> f32 {
    match profile {
        Profile::Web | Profile::Oltp | Profile::Dw => 1.0,
        Profile::Mixed => 0.5,
        Profile::Desktop => 0.2,
    }
}

/// The share of a profile's memory that all connections together may use for
/// `work_mem`.
fn profile_buffers_share(profile: Profile) -> f32 {
    match profile {
        Profile::Web => 0.25,
        Profile::Oltp => 0.35,
        Profile::Dw => 0.50,
        Profile::Mixed => 0.2,
        Profile::Desktop => 0.1,
    }
}

pub(crate) fn compute(facts: &Facts) -> Computed {
    let version = facts.version;
    let memory = facts.total_ram as f32 * profile_memory_share(facts.profile);

    // Memory. Inspired by
    // https://www.enterprisedb.com/postgres-tutorials/how-tune-postgresql-memory
    let unlimited_shared_buffers = (memory * 0.25) as i64;
    let mut shared_buffers = Memory::new(unlimited_shared_buffers);
    let effective_cache_size = (memory * 0.75) as i64;
    let mut work_mem = Memory::new(
        (memory * profile_buffers_share(facts.profile) / facts.max_connections as f32) as i64,
    );
    let mut maintenance_work_mem = Memory::new((memory * 0.05) as i64);

    if facts.thirty_two_bit {
        shared_buffers.limit(Limit::ThirtyTwoBit, 4 * GB);
        work_mem.limit(Limit::ThirtyTwoBit, 4 * GB);
        maintenance_work_mem.limit(Limit::ThirtyTwoBit, 4 * GB);
    }
    if version <= 9.6 {
        shared_buffers.limit(Limit::Through96, 512 * MB);
    }
    if facts.windows && version < 18.0 {
        work_mem.limit(Limit::WindowsBefore18, WINDOWS_MAX_WORK_MEM);
        maintenance_work_mem.limit(Limit::WindowsBefore18, WINDOWS_MAX_WORK_MEM);
    }
    let desktop_shared_buffers = facts.profile == Profile::Desktop;
    if desktop_shared_buffers {
        shared_buffers = Memory::new(facts.total_ram / 16);
    }
    if version < 9.6 {
        // https://github.com/postgres/postgres/commit/48354581a49c30f5757c203415aa8412d85b0f70
        shared_buffers.limit(Limit::Before96, 8 * GB);
    }

    // Checkpoints. wal_buffers of -1 lets PostgreSQL size it. Write-heavy
    // profiles get a fixed size, judged on shared_buffers before any limit.
    let wal_buffers = match facts.profile {
        Profile::Dw => 64 * MB,
        Profile::Oltp if unlimited_shared_buffers > 8 * GB => 32 * MB,
        _ => -1,
    };
    let (min_wal_size, max_wal_size) = match facts.profile {
        Profile::Dw => (4 * GB, 16 * GB),
        Profile::Oltp => (2 * GB, 8 * GB),
        Profile::Web => (GB, 4 * GB),
        Profile::Mixed => (2 * GB, 6 * GB),
        Profile::Desktop => (512 * MB, 2 * GB),
    };
    let wal_size = version >= 9.5;

    // Storage.
    let effective_io_concurrency = match facts.disk {
        Disk::Ssd => 200,
        Disk::San => 300,
        Disk::Hdd | Disk::Unrecognized => 2,
    };
    let random_page_cost = match (facts.disk, facts.profile) {
        (Disk::Hdd, _) => 4.0,
        // Analytical queries often read more than a tenth of a table, where a
        // sequential scan beats an index scan.
        (_, Profile::Dw) => 1.8,
        _ => 1.1,
    };

    // Workers.
    let max_worker_processes = facts.total_cpu.max(8);
    let max_parallel_workers = facts.total_cpu.max(8);
    let max_parallel_workers_per_gather = match facts.profile {
        Profile::Dw => (facts.total_cpu / 2).min(max_parallel_workers).max(2),
        _ => 2,
    };

    Computed {
        shared_buffers,
        desktop_shared_buffers,
        effective_cache_size,
        work_mem,
        maintenance_work_mem,
        min_wal_size: wal_size.then_some(min_wal_size),
        max_wal_size: wal_size.then_some(max_wal_size),
        checkpoint_completion_target: 0.9,
        wal_buffers,
        checkpoint_segments: (version <= 9.4).then_some(16),
        listen_addresses: "*",
        max_connections: facts.max_connections,
        random_page_cost,
        effective_io_concurrency,
        maintenance_io_concurrency: (version >= 13.0).then_some(effective_io_concurrency),
        aio: (version >= 18.0).then(|| aio(facts)),
        max_worker_processes: (version >= 9.4).then_some(max_worker_processes),
        max_parallel_workers_per_gather: (version >= 9.6)
            .then_some(max_parallel_workers_per_gather),
        max_parallel_workers: (version >= 10.0).then_some(max_parallel_workers),
    }
}

fn aio(facts: &Facts) -> Aio {
    let mut factor: f64 = match facts.profile {
        Profile::Desktop => 0.1,
        Profile::Web => 0.2,
        Profile::Mixed => 0.25,
        Profile::Oltp => 0.3,
        Profile::Dw => 0.4,
    };
    // A spinning disk gains from more workers.
    let hdd_adjusted = facts.disk == Disk::Hdd;
    if hdd_adjusted {
        factor += 0.1;
    }

    let initial = (facts.total_cpu as f64 * factor).ceil() as i64;
    let mut value = initial;
    let raised_to_minimum = value < 2;
    if raised_to_minimum {
        value = 2;
    }
    let capped_at_cpus = value > facts.total_cpu;
    if capped_at_cpus {
        value = facts.total_cpu;
    }
    let capped_at_maximum = value > MAX_IO_WORKERS;
    if capped_at_maximum {
        value = MAX_IO_WORKERS;
    }

    // The limits count 8kB pages: 16 is 128kB and 128 is 1MB.
    let (io_max_combine_limit, io_max_concurrency) = match facts.profile {
        Profile::Dw => (128, 256),
        Profile::Oltp => (16, 128),
        _ => (16, 64),
    };

    let dynamic_pool = facts.version >= 19.0;
    Aio {
        io_method: "worker",
        io_workers: (!dynamic_pool).then_some(IoWorkers {
            value,
            initial,
            hdd_adjusted,
            raised_to_minimum,
            capped_at_cpus,
            capped_at_maximum,
        }),
        // PostgreSQL 19 grows and shrinks the pool with demand. Keep its
        // defaults instead of applying the fixed-pool CPU formula.
        io_min_workers: dynamic_pool.then_some(2),
        io_max_workers: dynamic_pool.then_some(8),
        io_max_combine_limit: if dynamic_pool && facts.windows {
            io_max_combine_limit.min(16)
        } else {
            io_max_combine_limit
        },
        io_max_concurrency,
        file_copy_method: "copy",
    }
}

/// A computed value, typed the way PostgreSQL reads it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Value {
    Bytes(i64),
    Int(i64),
    Float(f32),
    Text(&'static str),
}

impl Value {
    /// The value as it is written in `postgresql.conf`, without quotes.
    pub(crate) fn render(self) -> String {
        match self {
            Value::Bytes(bytes) => crate::bytes::rounded(bytes),
            Value::Int(number) => number.to_string(),
            Value::Float(number) => format!("{number:.1}"),
            Value::Text(text) => text.to_string(),
        }
    }
}

/// Settings that belong together, in the order every output lists them.
pub(crate) struct Group {
    pub id: &'static str,
    pub description: &'static str,
    pub settings: Vec<(&'static str, Value)>,
}

impl Computed {
    /// The settings this PostgreSQL release has, grouped. A group can be
    /// empty: before PostgreSQL 9.4 there is no worker setting.
    pub(crate) fn groups(&self) -> Vec<Group> {
        let int = |value: Option<i64>| value.map(Value::Int);
        let bytes = |value: Option<i64>| value.map(Value::Bytes);
        let aio = self.aio;
        let group = |id, description, settings: Vec<(&'static str, Option<Value>)>| Group {
            id,
            description,
            settings: settings
                .into_iter()
                .filter_map(|(name, value)| value.map(|value| (name, value)))
                .collect(),
        };

        vec![
            group(
                "memory_related",
                "Memory Configuration",
                vec![
                    ("shared_buffers", bytes(Some(self.shared_buffers.value))),
                    (
                        "effective_cache_size",
                        bytes(Some(self.effective_cache_size)),
                    ),
                    ("work_mem", bytes(Some(self.work_mem.value))),
                    (
                        "maintenance_work_mem",
                        bytes(Some(self.maintenance_work_mem.value)),
                    ),
                ],
            ),
            group(
                "checkpoint_related",
                "Checkpoint Related Configuration",
                vec![
                    ("min_wal_size", bytes(self.min_wal_size)),
                    ("max_wal_size", bytes(self.max_wal_size)),
                    (
                        "checkpoint_completion_target",
                        Some(Value::Float(self.checkpoint_completion_target)),
                    ),
                    ("wal_buffers", bytes(Some(self.wal_buffers))),
                    ("checkpoint_segments", int(self.checkpoint_segments)),
                ],
            ),
            group(
                "network_related",
                "Network Related Configuration",
                vec![
                    ("listen_addresses", Some(Value::Text(self.listen_addresses))),
                    ("max_connections", int(Some(self.max_connections))),
                ],
            ),
            group(
                "storage_type",
                "Storage Configuration",
                vec![
                    (
                        "random_page_cost",
                        Some(Value::Float(self.random_page_cost)),
                    ),
                    (
                        "effective_io_concurrency",
                        int(Some(self.effective_io_concurrency)),
                    ),
                    (
                        "maintenance_io_concurrency",
                        int(self.maintenance_io_concurrency),
                    ),
                    ("io_method", aio.map(|aio| Value::Text(aio.io_method))),
                    (
                        "io_workers",
                        aio.and_then(|aio| aio.io_workers)
                            .map(|workers| Value::Int(workers.value)),
                    ),
                    (
                        "io_min_workers",
                        int(aio.and_then(|aio| aio.io_min_workers)),
                    ),
                    (
                        "io_max_workers",
                        int(aio.and_then(|aio| aio.io_max_workers)),
                    ),
                    (
                        "io_max_combine_limit",
                        aio.map(|aio| Value::Int(aio.io_max_combine_limit)),
                    ),
                    (
                        "io_max_concurrency",
                        aio.map(|aio| Value::Int(aio.io_max_concurrency)),
                    ),
                    (
                        "file_copy_method",
                        aio.map(|aio| Value::Text(aio.file_copy_method)),
                    ),
                ],
            ),
            group(
                "worker_related",
                "Worker Processes Configuration",
                vec![
                    ("max_worker_processes", int(self.max_worker_processes)),
                    (
                        "max_parallel_workers_per_gather",
                        int(self.max_parallel_workers_per_gather),
                    ),
                    ("max_parallel_workers", int(self.max_parallel_workers)),
                ],
            ),
        ]
    }
}
