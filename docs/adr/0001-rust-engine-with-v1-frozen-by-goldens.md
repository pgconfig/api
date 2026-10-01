---
status: accepted
date: 2026-10-01
supersedes: []
superseded_by: null
tags: [architecture, compatibility]
---

# The engine is a Rust crate, and REST v1 is frozen by goldens

pgconfig is rewritten in Rust as one Cargo workspace: the `pgconfig` crate
computes, and `pgconfig-server` and `pgconfigctl` are thin interfaces over it.
REST v1 and the CLI stay as public contracts, and their behavior is pinned by
golden files recorded from the last Go binaries, defects included. The reasons
for Rust are in `docs/research/rust-migration-assessment.md`.

> **Append-only:** never edit an accepted ADR. To change a decision, write a
> new ADR and link it to the old one via `supersedes` / `superseded_by`.

## Considered options

- **Fix the v1 defects during the port.** Rejected: installers call v1
  unattended at install time, and a float version or an accepted drive type
  that starts failing breaks them with nobody watching.
- **Port the Go engine as it is and expose only v1.** Rejected: epic #43 needs
  reasons, assumptions, and warnings, and the Go model cannot say why it chose
  a value.
- **One set of rules with two entry points.** Chosen. `tune` is strict and
  rich. The `v1` module feeds the same rules the loose inputs v1 always
  accepted and renders the old output.

## Consequences

- The known v1 defects live in `crates/pgconfig/src/v1` and nowhere else: the
  version read as a `float32`, the Windows rules that need the exact text
  `windows`, the unrecognized drive type, `listen_addresses = '*'`, and HTTP
  500 for every invalid input. Fixing one means a new API version, not a patch.
- A change to a rule changes the goldens. Record them again and review the
  diff, as `tests/golden/README.md` describes.
- The rules compare versions on the float scale v1 defined. `tune` only passes
  supported major versions, for which that scale is exact.
- The goldens leave out what Go itself did not do consistently. The list is in
  `tests/golden/README.md`.
