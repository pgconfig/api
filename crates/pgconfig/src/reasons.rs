//! The reason attached to each recommendation: one or two English sentences
//! that state the final value and how it was reached.

use crate::bytes::rounded;
use crate::request::{Os, Profile};
use crate::rules::{Computed, IoWorkers, Limit, Memory};
use crate::tune::NormalizedRequest;

pub(crate) fn reason(
    name: &str,
    value: &str,
    request: &NormalizedRequest,
    computed: &Computed,
) -> String {
    let profile = request.profile;
    let disk = request.disk_type;
    let cpus = request.total_cpu;
    match name {
        "shared_buffers" if computed.desktop_shared_buffers => {
            let base =
                format!("Set to {value} as one sixteenth of total RAM for the {profile} profile.");
            match limits(&computed.shared_buffers) {
                Some(limits) => format!("{base} Then {}", lowercase_first(&limits)),
                None => base,
            }
        }
        "shared_buffers" => limited(value, &computed.shared_buffers).unwrap_or_else(|| {
            format!("Set to {value} from the memory share for the {profile} profile.")
        }),
        "effective_cache_size" => format!(
            "Set to {value} from memory available to the PostgreSQL and operating-system caches."
        ),
        "work_mem" => limited(value, &computed.work_mem).unwrap_or_else(|| {
            format!(
                "Set to {value} from available memory for the {profile} profile and {} connections.",
                request.max_connections
            )
        }),
        "maintenance_work_mem" => {
            limited(value, &computed.maintenance_work_mem).unwrap_or_else(|| {
                format!("Set to {value} as 5% of memory available to the {profile} profile.")
            })
        }
        "min_wal_size" | "max_wal_size" | "io_max_combine_limit" | "io_max_concurrency" => {
            format!("Set to {value} for the {profile} workload profile.")
        }
        "checkpoint_completion_target" => format!(
            "Set to {value} to spread checkpoint I/O across most of the checkpoint interval."
        ),
        "wal_buffers" => match (profile, computed.wal_buffers) {
            (Profile::Dw, _) => format!("Set to {value} for write-heavy DW workloads."),
            (Profile::Oltp, size) if size > 0 => format!(
                "Set to {value} for OLTP because the profile's share of memory for shared_buffers exceeds 8GB."
            ),
            _ => format!(
                "Set to {value} to use automatic PostgreSQL tuning for the {profile} workload profile."
            ),
        },
        "checkpoint_segments" => format!(
            "Set to {value} as the legacy checkpoint segment count for PostgreSQL releases before 9.5."
        ),
        "max_connections" => format!("Set to {value} to match the requested connection limit."),
        "random_page_cost" => {
            format!("Set to {value} for {disk} storage and the {profile} workload profile.")
        }
        // The Windows rule set this to 0 and the storage rule then replaced
        // it, as it always has.
        "effective_io_concurrency" if request.os == Os::Windows => format!(
            "Set to {value} for the requested {disk} storage; the storage policy replaces the initial Windows compatibility value."
        ),
        "effective_io_concurrency" | "maintenance_io_concurrency" => {
            format!("Set to {value} for the requested {disk} storage.")
        }
        "io_method" => format!("Set to {value} to use worker-based asynchronous I/O."),
        "io_workers" => match computed.aio {
            Some(aio) => io_workers(value, request, &aio.io_workers),
            None => String::new(),
        },
        "file_copy_method" => format!("Set to {value} as the copy method for file operations."),
        "max_worker_processes" | "max_parallel_workers" if cpus < 8 => {
            format!("Set to {value} by applying the minimum of 8 to {cpus} logical CPUs.")
        }
        "max_worker_processes" | "max_parallel_workers" => {
            format!("Set to {value} to match the {cpus} logical CPUs.")
        }
        "max_parallel_workers_per_gather" if profile != Profile::Dw => format!(
            "Set to {value} as the parallel-query default for the {profile} workload profile."
        ),
        "max_parallel_workers_per_gather" if cpus / 2 < 2 => format!(
            "Set to {value} from half of {cpus} logical CPUs, raised to the minimum of 2 for the DW workload profile."
        ),
        "max_parallel_workers_per_gather" => {
            format!("Set to {value} from half of {cpus} logical CPUs for the DW workload profile.")
        }
        _ => format!("Set to {value} by the tuning policy for the {profile} workload profile."),
    }
}

/// The reason of a memory setting that a limit lowered, or `None` when no
/// limit applied.
fn limited(value: &str, memory: &Memory) -> Option<String> {
    limits(memory).map(|limits| format!("Set to {value}. {limits}"))
}

/// One sentence per limit, in the order they applied: "Capped from 16GB to
/// 4GB for the 32-bit architecture. Then capped from 4GB to 512MB for ...".
fn limits(memory: &Memory) -> Option<String> {
    if memory.limits.is_empty() {
        return None;
    }
    // Each limit recorded the value it replaced. The value it left behind is
    // what the next limit replaced, or the final value.
    let after = memory
        .limits
        .iter()
        .skip(1)
        .map(|(_, before)| *before)
        .chain([memory.value]);
    let sentences: Vec<String> = memory
        .limits
        .iter()
        .zip(after)
        .enumerate()
        .map(|(index, ((limit, before), after))| {
            let (before, after) = (rounded(*before), rounded(after));
            let context = match limit {
                Limit::ThirtyTwoBit => "for the 32-bit architecture",
                Limit::Through96 => "for PostgreSQL 9.6 and older",
                Limit::Before96 => "for PostgreSQL releases before 9.6",
                Limit::WindowsBefore18 => "by the Windows limit of PostgreSQL releases before 18",
            };
            let capped = if before == after {
                format!("Capped at {after} {context}.")
            } else {
                format!("Capped from {before} to {after} {context}.")
            };
            if index == 0 {
                capped
            } else {
                format!("Then {}", lowercase_first(&capped))
            }
        })
        .collect();
    Some(sentences.join(" "))
}

fn lowercase_first(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn io_workers(value: &str, request: &NormalizedRequest, workers: &IoWorkers) -> String {
    let cpus = match request.total_cpu {
        1 => "1 logical CPU".to_string(),
        count => format!("{count} logical CPUs"),
    };
    let mut reason = format!(
        "Set to {value} from {cpus} and the {} profile",
        request.profile
    );
    if workers.hdd_adjusted {
        reason.push_str(" with the HDD storage adjustment");
    }
    reason.push('.');

    let bounds = [
        (
            workers.raised_to_minimum,
            "raised to the minimum of 2".to_string(),
        ),
        (
            workers.capped_at_cpus,
            format!(
                "capped at {} to avoid exceeding the logical CPU count",
                request.total_cpu
            ),
        ),
        (
            workers.capped_at_maximum,
            "capped at 32, the PostgreSQL maximum".to_string(),
        ),
    ];
    for (index, (_, bound)) in bounds.iter().filter(|(applied, _)| *applied).enumerate() {
        if index == 0 {
            reason.push_str(&format!(
                " The initial {}-worker calculation was {bound}.",
                workers.initial
            ));
        } else {
            reason.push_str(&format!(" It was then {bound}."));
        }
    }
    reason
}
