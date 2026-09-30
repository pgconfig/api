import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const [command, expectedVersion] = process.argv.slice(2);
const { version } = JSON.parse(readFileSync("package.json", "utf8"));
assert.match(version, /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/, "Invalid release version");

if (command === "check") {
  if (expectedVersion) assert.equal(version, expectedVersion, "Tag and package version differ");
} else if (command === "notes") {
  const changelog = readFileSync("CHANGELOG.md", "utf8");
  const heading = `## ${version}\n`;
  const start = changelog.indexOf(heading);
  assert.ok(start >= 0, `Missing changelog entry for ${version}`);
  const rest = changelog.slice(start + heading.length);
  const end = rest.indexOf("\n## ");
  const notes = (end < 0 ? rest : rest.slice(0, end)).trim();
  assert.ok(notes, `Empty changelog entry for ${version}`);
  console.log(notes);
} else {
  throw new Error("Usage: release-version.mjs check [version] | notes");
}
