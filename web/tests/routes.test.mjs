import assert from "node:assert/strict";
import { test } from "node:test";
import {
  crumbsFor,
  guideSlug,
  isGuidePath,
  isMcpPath,
  isTuningPath,
} from "../src/lib/routes.ts";

const titles = { api: "Overview", mcp: "MCP" };
const titleOf = (slug) => titles[slug];

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

test("the MCP page has its own place in the sidebar, apart from the docs", () => {
  assert.strictEqual(isMcpPath("/guide/mcp"), true);
  assert.strictEqual(isMcpPath("/guide/mcp/"), true);
  assert.strictEqual(isMcpPath("/guide"), false);
  assert.strictEqual(isMcpPath("/guide/api"), false);
  assert.strictEqual(isMcpPath("/mcp"), false);
});

test("a guide address names its page, and the index has no name", () => {
  assert.strictEqual(guideSlug("/guide"), "");
  assert.strictEqual(guideSlug("/guide/"), "");
  assert.strictEqual(guideSlug("/guide/mcp"), "mcp");
  assert.strictEqual(guideSlug("/guide/mcp/"), "mcp");
});

test("the comparison is named alone, at either address", () => {
  const expected = [{ label: "Profile comparison" }];
  assert.deepStrictEqual(crumbsFor("/", titleOf), expected);
  assert.deepStrictEqual(crumbsFor("/tuning", titleOf), expected);
});

test("the export page is named alone", () => {
  assert.deepStrictEqual(crumbsFor("/export", titleOf), [{ label: "Export" }]);
});

test("the docs index is named alone", () => {
  assert.deepStrictEqual(crumbsFor("/guide", titleOf), [{ label: "Docs" }]);
  assert.deepStrictEqual(crumbsFor("/guide/", titleOf), [{ label: "Docs" }]);
});

test("a docs page sits under the docs, by its title", () => {
  assert.deepStrictEqual(crumbsFor("/guide/mcp", titleOf), [
    { label: "Docs", to: "/guide" },
    { label: "MCP" },
  ]);
});

test("an address with no page is named as not found", () => {
  assert.deepStrictEqual(crumbsFor("/guide/missing", titleOf), [
    { label: "Docs", to: "/guide" },
    { label: "Not found" },
  ]);
  assert.deepStrictEqual(crumbsFor("/nope", titleOf), [{ label: "Not found" }]);
});
