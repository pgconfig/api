# Releases

Changesets keeps one version PR open. Merging it tags the version and runs the
Release workflow, which publishes the GitHub release, the binaries, the deb and
rpm packages, and the Docker images on GHCR. Nothing is published to npm or to
crates.io.

Releases use `v` tags, such as `v3.7.0`. Releases up to `3.6.1` used bare tags.

## Add release notes

From the repository root:

```sh
npm ci
npm run changeset
```

Select `pgconfig-api`, choose the version bump, and describe the user-visible
change in English. Commit the generated `.changeset/*.md` with the change.
Use a patch for fixes and a minor for new behavior. Changesets combines pending
entries into the next version PR. CI and documentation-only changes need no
changeset.

On `main`, the Version workflow opens or updates `chore: release pgconfig-api`.
Review its changelog and version, wait for CI, then merge it. The PR updates
`package.json`, `package-lock.json`, `CHANGELOG.md`, `Cargo.toml`, and
`Cargo.lock`. Do not bump these versions by hand.

## Where the version comes from

Changesets owns the version in `package.json`. The binaries read theirs from
the Cargo workspace, so `npm run version:packages` runs
`scripts/release-version.mjs sync` to copy it into `Cargo.toml` and
`Cargo.lock`. `scripts/release-version.mjs check` fails when they differ.

The release build sets `PGCONFIG_COMMIT`, so a released binary reports
`3.7.0 (<commit>)`. A local build reports `3.7.0 (development)`.

## What the Release workflow does

1. Five jobs compile `pgconfigctl` and `pgconfig-server`, one per target, each
   on a runner of its own platform: Linux on x86-64 and arm64 (static, musl),
   macOS on x86-64 and arm64, and Windows on x86-64. Each job builds the web
   app first, because the server embeds it.
2. One job downloads those binaries and runs GoReleaser, which packages them
   without compiling. `scripts/release-cargo.sh` hands each binary to
   GoReleaser's Rust builder.

The Release workflow extracts the version's summary from `CHANGELOG.md` and
passes it to GoReleaser with `--release-notes`. Keep `changelog.disable` unset
in `.goreleaser.yml`: disabling the changelog also prevents GoReleaser from
reading the supplied notes.

## Docker images

- `ghcr.io/momoi-labs/pgconfig` runs `pgconfig-server`.
- `ghcr.io/momoi-labs/pgconfigctl` runs `pgconfigctl`.

Both are built from `scratch` with the static binary and run as a non-root
user. The images `pgconfig/api` and `pgconfig/pgconfigctl`, on Docker Hub and
on `ghcr.io/pgconfig`, stopped at 3.6.1.

## Workflow tokens

The Version workflow uses the built-in `GITHUB_TOKEN`. GitHub does not trigger
workflows from events created with that token, so the workflow dispatches
Verify on the version PR branch and calls the Release workflow after tagging.

The repository setting **Settings > Actions > General > Allow GitHub Actions to
create and approve pull requests** must stay enabled.

## Retry a release

In **Actions > release > Run workflow**, enter an existing `v` tag to retry a
failed release. The workflow checks out that tag, checks the version, and uses
its changelog. Re-running the Version workflow alone will not retry a tag it has
already created. Tags before the Rust cutover cannot be retried this way.

## Verification

`mise install` provides every tool used here. Run these checks before merging
release changes:

```sh
npm ci
npm run test:release
goreleaser check
actionlint
```

The tests exercise Changesets in a temporary Git repository. They verify that
the Cargo workspace follows the package version, and that tagging a version
twice, or a version that already has a bare tag, does not publish again.

To rehearse the build, compile the Linux and macOS targets as a snapshot:

```sh
just release-build
```

It needs the Rust targets once:

```sh
rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl \
  x86_64-apple-darwin aarch64-apple-darwin
```

The Windows target compiles only on its own runner, so a full
`goreleaser release --snapshot` does not complete on a Mac or on Linux. The
packaging of archives, packages, and images runs for the first time in the
Release workflow.
