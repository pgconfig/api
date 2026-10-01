//! The tuning operation: a Tuning Request in, a Tuning Result out.

use std::collections::BTreeMap;
use std::num::NonZeroU32;

use serde::Serialize;

use crate::bytes::Bytes;
use crate::reasons;
use crate::request::{Arch, DiskType, Os, Profile, TuningRequest};
use crate::rules::{self, Disk, Facts};
use crate::version::PgVersion;

/// The request the recommendations were computed from: every supplied fact
/// normalized, and every omitted one filled in.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NormalizedRequest {
    pub os: Os,
    pub arch: Arch,
    pub total_ram: Bytes,
    pub profile: Profile,
    pub disk_type: DiskType,
    pub max_connections: u32,
    pub total_cpu: u32,
    pub postgres_version: PgVersion,
}

/// A recommended value of one PostgreSQL parameter, and why.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TuningRecommendation {
    /// The value as it is written in `postgresql.conf`.
    pub value: String,
    /// One or two sentences on how the value was reached, including any limit
    /// that changed it.
    pub reason: String,
}

/// A fact the caller did not supply and `tune` filled in. No default is
/// applied silently: each one shows up here.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TuningAssumption {
    /// The request field that was omitted.
    pub field: &'static str,
    /// The value assumed for it.
    pub value: String,
    pub message: String,
}

/// A concern about a usable request. A warning never invalidates the result.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TuningWarning {
    /// A stable identifier of the kind of concern.
    pub code: &'static str,
    pub message: String,
}

/// What `tune` returns.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TuningResult {
    pub request: NormalizedRequest,
    pub assumptions: Vec<TuningAssumption>,
    pub warnings: Vec<TuningWarning>,
    /// Keyed by PostgreSQL parameter name. A parameter the release does not
    /// have is absent.
    pub recommendations: BTreeMap<&'static str, TuningRecommendation>,
}

const DEFAULT_MAX_CONNECTIONS: u32 = 100;

/// Above this many connections the result carries a warning. No count is
/// rejected: unusual deployments stay representable.
const HIGH_MAX_CONNECTIONS: u32 = 1000;

/// Computes the recommendations for a request. The same request always gives
/// the same result.
pub fn tune(request: &TuningRequest) -> TuningResult {
    let mut assumptions = Vec::new();
    let profile = resolve(&mut assumptions, "profile", request.profile, Profile::Web);
    let disk_type = resolve(
        &mut assumptions,
        "disk_type",
        request.disk_type,
        DiskType::Ssd,
    );
    let os = resolve(&mut assumptions, "os", request.os, Os::Linux);
    let arch = resolve(&mut assumptions, "arch", request.arch, Arch::Amd64);
    let max_connections = resolve(
        &mut assumptions,
        "max_connections",
        request.max_connections.map(NonZeroU32::get),
        DEFAULT_MAX_CONNECTIONS,
    );
    let normalized = NormalizedRequest {
        os,
        arch,
        total_ram: request.total_ram,
        profile,
        disk_type,
        max_connections,
        total_cpu: request.total_cpu.get(),
        postgres_version: request.postgres_version.clone(),
    };

    let mut warnings = Vec::new();
    if normalized.max_connections > HIGH_MAX_CONNECTIONS {
        warnings.push(TuningWarning {
            code: "high_max_connections",
            message: format!(
                "max_connections is {}. Every connection takes memory, and work_mem is divided among them. Above {HIGH_MAX_CONNECTIONS} connections, consider a connection pooler in front of PostgreSQL.",
                normalized.max_connections
            ),
        });
    }

    let computed = rules::compute(&Facts {
        total_ram: normalized.total_ram.get(),
        total_cpu: normalized.total_cpu.into(),
        max_connections: normalized.max_connections.into(),
        profile: normalized.profile,
        disk: match normalized.disk_type {
            DiskType::Hdd => Disk::Hdd,
            DiskType::Ssd => Disk::Ssd,
            DiskType::San => Disk::San,
        },
        windows: normalized.os == Os::Windows,
        thirty_two_bit: normalized.arch == Arch::X86,
        version: normalized.postgres_version.major().rule_scale(),
    });

    let recommendations = computed
        .groups()
        .into_iter()
        .flat_map(|group| group.settings)
        // listen_addresses is about who may connect, not about performance,
        // and '*' is not a safe thing to recommend. Only v1 keeps it.
        .filter(|(name, _)| *name != "listen_addresses")
        .map(|(name, value)| {
            let value = value.render();
            let reason = reasons::reason(name, &value, &normalized, &computed);
            (name, TuningRecommendation { value, reason })
        })
        .collect();

    TuningResult {
        request: normalized,
        assumptions,
        warnings,
        recommendations,
    }
}

/// The supplied value, or the default with an assumption recorded for it.
fn resolve<T: ToString>(
    assumptions: &mut Vec<TuningAssumption>,
    field: &'static str,
    supplied: Option<T>,
    default: T,
) -> T {
    supplied.unwrap_or_else(|| {
        let value = default.to_string();
        let message = format!("{field} was not supplied. Assumed {value}.");
        assumptions.push(TuningAssumption {
            field,
            value,
            message,
        });
        default
    })
}
