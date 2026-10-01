//! PostgreSQL configuration tuning.
//!
//! The crate turns the facts about a server (memory, CPUs, PostgreSQL
//! version, workload) into recommended settings. It does no I/O.
//!
//! [`tune`] is the entry point: it takes a [`TuningRequest`] and returns a
//! [`TuningResult`]. [`v1`] reproduces the output of REST v1 and
//! `pgconfigctl` from the same rules.

pub mod build;
mod bytes;
mod docs;
mod reasons;
mod request;
mod rules;
mod tune;
pub mod v1;
mod version;

pub use bytes::{Bytes, BytesError};
pub use request::{
    Arch, DiskType, Os, Problem, Profile, RawTuningRequest, TuningError, TuningRequest,
};
pub use tune::{
    NormalizedRequest, TuningAssumption, TuningRecommendation, TuningResult, TuningWarning, tune,
};
pub use version::{PgMajor, PgVersion, PgVersionError};
