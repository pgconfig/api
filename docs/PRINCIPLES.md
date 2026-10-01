# Engineering Principles

These principles govern *how* this repository is built. They are not a
specification of *what* it does — that lives in the issue tracker — and not a
glossary of domain terms — that lives in `CONTEXT.md`.

Use them when an implementation choice has more than one correct answer. When
a principle and a convenience conflict, the principle wins. When two principles
conflict, the order below is the tie-breaker.

## 1. Smallest viable change

Change only what the task requires, so each pull request stays as small as it
can be. The goal is not a small diff for its own sake: a small, focused change
is easier to review. The people reading the code — including AI-generated code —
can see exactly what moved and decide whether to keep it or adapt it.

When choosing between two correct implementations, prefer the one that:

- changes fewer files and fewer lines to reach the same outcome;
- keeps one concern per change and splits unrelated concerns into separate
  changes;
- leaves existing behavior and formatting untouched where the task does not
  require touching them;
- defers anything speculative the task does not ask for.

## 2. Commit history tells a story

The commit history is a narrative, not a record of saves. A reader coming later
should be able to reconstruct what happened and how the code got here — the
problems met and the paths chosen. Write every commit through `my-commit`, with
a conventional header and a body that explains the *why*, not just the *what*.

When writing a commit, prefer the one that:

- explains the problem being solved and the reasoning, not just the diff;
- keeps the subject a crisp, conventional summary and moves detail into the body;
- records the decision and its trade-off, because the diff alone does not;
- stands on its own when read in sequence with the commits around it.

## 3. Interfaces are a compatibility contract

A published interface — an API, a file format, a schema, a CLI — outlives the
code that produced it. Consumers depend on it, so changes to it are governed,
not free:

- **Breaking changes require a new major version.** Removing, renaming,
  retyping, or changing the meaning of something counts as breaking.
- **Additive changes are minor versions.** New fields, endpoints, and columns
  may land in a minor version; readers must tolerate what they do not recognize.
- **Every new major must ship one of two things**, in order of preference:
  1. a migration that upgrades the previous major, or
  2. the ability to read the previous major directly.
- **Compatibility flows one direction only:** a newer reader supports older
  artifacts; an older reader is never expected to understand a newer artifact.

## 4. No silent defaults

Any behavior that carries a cost — performance, storage, a side effect, an
opinion the user did not ask for — must be **opt-in and visible**, never a
silent default. A costly or surprising behavior enabled without the user's
knowledge is a defect, even when the behavior is useful.

When a default must exist, prefer the conservative one and document what the
other options cost.

## 5. Decisions are recorded, append-only

Record every decision a future reader **cannot recover from the code alone** —
the "why we chose X and not Y" that the diff does not show. If the code already
makes the choice obvious, it needs no ADR; if it doesn't, it does.

ADRs are **append-only**: an accepted decision is never rewritten. To change a
decision, write a new ADR that supersedes the old one (via `supersedes` /
`superseded_by`) and leave the old ADR in place as the record of what changed
and why. The sequence of ADRs is the story of the decisions.
