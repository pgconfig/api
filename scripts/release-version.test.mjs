import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { cpSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { test } from "node:test";

const script = resolve("scripts/release-version.mjs");
const crates = ["pgconfig", "pgconfig-golden", "pgconfig-server", "pgconfigctl"];
function cargoFiles(cwd, version) {
  writeFileSync(
    join(cwd, "Cargo.toml"),
    `[workspace]\nmembers = ["crates/*"]\n\n[workspace.package]\nversion = "${version}"\nedition = "2024"\n\n[workspace.dependencies]\nserde = { version = "1" }\n`,
  );
  const packages = ["serde", ...crates].map((name) =>
    `[[package]]\nname = "${name}"\nversion = "${name === "serde" ? "1.0.229" : version}"\n`);
  writeFileSync(join(cwd, "Cargo.lock"), `version = 4\n\n${packages.join("\n")}`);
}
function fixture(t) {
  const cwd = mkdtempSync(join(tmpdir(), "pgconfig-api-version-"));
  t.after(() => rmSync(cwd, { recursive: true, force: true }));
  writeFileSync(join(cwd, "package.json"), JSON.stringify({ version: "3.7.0" }));
  cargoFiles(cwd, "3.7.0");
  return cwd;
}
function run(cwd, ...args) {
  return spawnSync(process.execPath, [script, ...args], { cwd, encoding: "utf8" });
}

test("check rejects a tag that differs from the package version", (t) => {
  const cwd = fixture(t);
  assert.equal(run(cwd, "check").status, 0);
  assert.equal(run(cwd, "check", "3.7.0").status, 0);
  assert.notEqual(run(cwd, "check", "3.8.0").status, 0);
});

test("check rejects cargo files that differ from the package version", (t) => {
  const cwd = fixture(t);
  cargoFiles(cwd, "3.6.1");
  const result = run(cwd, "check");
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Cargo.toml version differs/);
});

test("sync writes the package version to the workspace and every crate in the lock", (t) => {
  const cwd = fixture(t);
  cargoFiles(cwd, "3.6.1");
  assert.equal(run(cwd, "sync").status, 0);
  assert.match(readFileSync(join(cwd, "Cargo.toml"), "utf8"), /\[workspace.package\]\nversion = "3.7.0"/);
  const lock = readFileSync(join(cwd, "Cargo.lock"), "utf8");
  for (const name of crates) {
    assert.ok(lock.includes(`name = "${name}"\nversion = "3.7.0"`), `${name} was not synced`);
  }
  // A dependency keeps its own version.
  assert.ok(lock.includes('name = "serde"\nversion = "1.0.229"'));
  assert.equal(run(cwd, "check").status, 0);
});

test("sync changes nothing when a crate is missing from the lock", (t) => {
  const cwd = fixture(t);
  cargoFiles(cwd, "3.6.1");
  const lock = readFileSync(join(cwd, "Cargo.lock"), "utf8").replace('name = "pgconfigctl"', 'name = "renamed"');
  writeFileSync(join(cwd, "Cargo.lock"), lock);
  const manifest = readFileSync(join(cwd, "Cargo.toml"), "utf8");
  assert.notEqual(run(cwd, "sync").status, 0);
  assert.equal(readFileSync(join(cwd, "Cargo.toml"), "utf8"), manifest);
  assert.equal(readFileSync(join(cwd, "Cargo.lock"), "utf8"), lock);
});

test("notes contains only the current release and requires an entry", (t) => {
  const cwd = fixture(t);
  writeFileSync(join(cwd, "CHANGELOG.md"), "# pgconfig-api\n\n## 3.7.0\n\n### Minor Changes\n\n- Changesets.\n\n## 3.6.1\n\n- Previous release.\n");
  assert.equal(run(cwd, "notes").stdout, "### Minor Changes\n\n- Changesets.\n");
  writeFileSync(join(cwd, "CHANGELOG.md"), "# pgconfig-api\n");
  assert.notEqual(run(cwd, "notes").status, 0);
});

test("the version workflow pushes the tag and hands it to the release only once", (t) => {
  const cwd = fixture(t);
  for (const path of ["package.json", "package-lock.json", "Cargo.toml", "Cargo.lock", ".changeset/config.json", "scripts/release-version.mjs"]) {
    cpSync(resolve(path), join(cwd, path), { recursive: true });
  }
  const pkg = JSON.parse(readFileSync(join(cwd, "package.json")));
  pkg.version = "3.6.1";
  writeFileSync(join(cwd, "package.json"), JSON.stringify(pkg));
  // The copied manifests carry whatever version the repository is at. The
  // scenario starts at 3.6.1, so the script itself brings them there.
  assert.equal(run(cwd, "sync").status, 0);
  symlinkSync(resolve("node_modules"), join(cwd, "node_modules"), "dir");
  const exec = (cmd, args, env = {}) => {
    const result = spawnSync(cmd, args, {
      cwd,
      encoding: "utf8",
      env: { ...process.env, npm_config_offline: "true", ...env },
    });
    assert.equal(result.status, 0, result.stderr || result.stdout);
    return result.stdout;
  };
  exec("git", ["init", "-b", "main"]);
  exec("git", ["-c", "user.name=Test", "-c", "user.email=test@example.com", "commit", "--allow-empty", "-m", "test"]);
  exec("git", ["init", "--bare", join(cwd, "remote.git")]);
  exec("git", ["remote", "add", "origin", join(cwd, "remote.git")]);
  const workflow = readFileSync(resolve(".github/workflows/changesets.yml"), "utf8");
  const match = workflow.match(/- name: Tag the merged version[\s\S]*?run: \|\n((?: {10}[^\n]*\n)+)/);
  assert.ok(match, "The workflow must have an explicit tagging step");
  const tagStep = match[1].replace(/^ {10}/gm, "");
  const output = join(cwd, "github-output");
  const env = { GITHUB_OUTPUT: output, GIT_COMMITTER_NAME: "Test", GIT_COMMITTER_EMAIL: "test@example.com" };

  // Versions released before Changesets have bare tags and must not publish again.
  exec("git", ["tag", "3.6.1"]);
  writeFileSync(output, "");
  exec("bash", ["-e", "-c", tagStep], env);
  assert.equal(readFileSync(output, "utf8"), "");
  assert.doesNotMatch(exec("git", ["ls-remote", "--tags", "origin"]), /v3.6.1/);

  writeFileSync(join(cwd, ".changeset/test.md"), '---\n"pgconfig-api": minor\n---\n\nRelease with Changesets.\n');
  exec("npm", ["run", "version:packages"]);
  const version = JSON.parse(readFileSync(join(cwd, "package.json"))).version;
  assert.equal(version, "3.7.0");
  assert.equal(run(cwd, "check", version).status, 0);
  assert.equal(run(cwd, "notes").status, 0);
  exec("bash", ["-e", "-c", tagStep], env);
  assert.equal(readFileSync(output, "utf8"), "tag=v3.7.0\n");
  assert.match(exec("git", ["ls-remote", "--tags", "origin"]), /refs\/tags\/v3.7.0/);
  writeFileSync(output, "");
  exec("bash", ["-e", "-c", tagStep], env);
  assert.equal(readFileSync(output, "utf8"), "");

  // A tag created locally must not start a release if pushing it fails.
  exec("git", ["tag", "-d", "v3.7.0"]);
  exec("git", ["remote", "set-url", "origin", join(cwd, "missing.git")]);
  const failed = spawnSync("bash", ["-e", "-c", tagStep], {
    cwd, encoding: "utf8", env: { ...process.env, ...env },
  });
  assert.notEqual(failed.status, 0);
  assert.equal(readFileSync(output, "utf8"), "");
});

test("the release build checks files out with LF on every runner", () => {
  // A Windows runner converts to CRLF by default, and the version check then
  // cannot find the version in Cargo.toml.
  const workflow = readFileSync(resolve(".github/workflows/release.yml"), "utf8");
  const build = workflow.slice(workflow.indexOf("\n  build:"), workflow.indexOf("\n  goreleaser:"));
  const setting = build.indexOf("git config --global core.autocrlf false");
  assert.ok(setting >= 0, "The build job must turn off line-ending conversion");
  assert.ok(setting < build.indexOf("actions/checkout"), "It must do so before the checkout");
});
