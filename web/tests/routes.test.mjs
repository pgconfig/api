import assert from "node:assert/strict";
import { test } from "node:test";
import { crumbsFor, guideSlug, isGuidePath, isTuningPath } from "../src/lib/routes.ts";

const titles = { api: "Overview", mcp: "MCP" };
const titleOf = (slug) => titles[slug];
const home = "/?cpus=4";
const root = { label: "PGConfig", to: home };

test("the comparison, its old address and the export page tune a server", () => {
  assert.strictEqual(isTuningPath("/"), true);
  assert.strictEqual(isTuningPath("/tuning"), true);
  assert.strictEqual(isTuningPath("/export"), true);
  assert.strictEqual(isTuningPath("/export/"), true);
});

test("the guide and unknown addresses do not tune anything", () => {
  assert.strictEqual(isTuningPath("/guide"), false);
  assert.strictEqual(isTuningPath("/guide/mcp"), false);
  assert.strictEqual(isTuningPath("/exports"), false);
});

test("only /guide and what is under it belongs to the guide", () => {
  assert.strictEqual(isGuidePath("/guide"), true);
  assert.strictEqual(isGuidePath("/guide/"), true);
  assert.strictEqual(isGuidePath("/guide/mcp"), true);
  assert.strictEqual(isGuidePath("/guidelines"), false);
  assert.strictEqual(isGuidePath("/"), false);
});

test("a guide address names its page, and the index has no name", () => {
  assert.strictEqual(guideSlug("/guide"), "");
  assert.strictEqual(guideSlug("/guide/"), "");
  assert.strictEqual(guideSlug("/guide/mcp"), "mcp");
  assert.strictEqual(guideSlug("/guide/mcp/"), "mcp");
});

test("the comparison is one step from the root, at either address", () => {
  const expected = [root, { label: "Profile comparison" }];
  assert.deepStrictEqual(crumbsFor("/", home, titleOf), expected);
  assert.deepStrictEqual(crumbsFor("/tuning", home, titleOf), expected);
});

test("the export page is one step from the root", () => {
  assert.deepStrictEqual(crumbsFor("/export", home, titleOf), [root, { label: "Export" }]);
});

test("the guide index is one step from the root", () => {
  assert.deepStrictEqual(crumbsFor("/guide", home, titleOf), [root, { label: "Guide" }]);
  assert.deepStrictEqual(crumbsFor("/guide/", home, titleOf), [root, { label: "Guide" }]);
});

test("a guide page sits under the guide, by its title", () => {
  assert.deepStrictEqual(crumbsFor("/guide/mcp", home, titleOf), [
    root,
    { label: "Guide", to: "/guide" },
    { label: "MCP" },
  ]);
});

test("an address with no page is named as not found", () => {
  assert.deepStrictEqual(crumbsFor("/guide/missing", home, titleOf), [
    root,
    { label: "Guide", to: "/guide" },
    { label: "Not found" },
  ]);
  assert.deepStrictEqual(crumbsFor("/nope", home, titleOf), [root, { label: "Not found" }]);
});
