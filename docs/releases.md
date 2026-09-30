# Releases

Changesets keeps one version PR open. Merging it tags the version and runs
GoReleaser, which publishes the GitHub release, binaries, deb and rpm packages,
and Docker images on Docker Hub and GHCR. Nothing is published to npm.

Releases use `v` tags, such as `v3.7.0`. Releases up to `3.6.1` used bare tags.
The root `package.json` starts at `3.6.1`, so the first changeset proposes the
next version after it.

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
`package.json`, `package-lock.json`, and `CHANGELOG.md`. Do not bump these
versions by hand.

GoReleaser injects the version into the binaries from the tag, so no Go file
holds it. The Release workflow extracts that version's summary from
`CHANGELOG.md` and passes it to GoReleaser with `--release-notes`. Keep
`changelog.disable` unset in `.goreleaser.yml`: disabling the changelog also
prevents GoReleaser from reading the supplied notes.

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
already created. Tags before `v3.7.0` cannot be retried this way.

## Verification

Run these checks before merging release changes:

```sh
npm ci
npm run test:release
goreleaser check
actionlint
```

The tests exercise Changesets in a temporary Git repository and verify that
tagging a version twice, or a version that already has a bare tag, does not
publish again.
