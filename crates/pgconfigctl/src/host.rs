//! The facts about the machine the CLI runs on, used when a flag is omitted.

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

/// The operating system, named the way the rules name it.
pub fn os() -> &'static str {
    rules_os(std::env::consts::OS)
}

/// The CPU architecture, named the way the rules name it.
pub fn arch() -> &'static str {
    rules_arch(std::env::consts::ARCH)
}

/// Total memory in bytes, and the number of logical CPUs.
pub fn memory_and_cpus() -> (i64, i64) {
    let system = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::nothing().with_ram())
            .with_cpu(CpuRefreshKind::nothing()),
    );
    let memory = i64::try_from(system.total_memory()).unwrap_or(i64::MAX);
    let cpus = i64::try_from(system.cpus().len()).unwrap_or(i64::MAX);
    (memory, cpus)
}

/// The rules use the Go names: `darwin` for macOS. Other systems keep their
/// name and the rules decide whether they know it.
fn rules_os(rust_name: &'static str) -> &'static str {
    match rust_name {
        "macos" => "darwin",
        other => other,
    }
}

/// The rules use the Go names: `amd64`, `arm64`, and `386`.
fn rules_arch(rust_name: &'static str) -> &'static str {
    match rust_name {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "x86" => "386",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_follow_the_rules() {
        assert_eq!(rules_os("macos"), "darwin");
        assert_eq!(rules_os("linux"), "linux");
        assert_eq!(rules_os("windows"), "windows");
        assert_eq!(rules_arch("x86_64"), "amd64");
        assert_eq!(rules_arch("aarch64"), "arm64");
        assert_eq!(rules_arch("x86"), "386");
        assert_eq!(rules_arch("arm"), "arm");
    }

    #[test]
    fn this_machine_has_memory_and_cpus() {
        let (memory, cpus) = memory_and_cpus();

        assert!(memory > 0);
        assert!(cpus > 0);
    }
}
