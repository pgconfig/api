import assert from "node:assert/strict";
import { test } from "node:test";
import { isDarkTheme, toggledTheme } from "../src/lib/theme.ts";

test("a stored choice decides whether the page is dark", () => {
  assert.strictEqual(isDarkTheme("dark", false), true);
  assert.strictEqual(isDarkTheme("light", true), false);
});

test("without a stored choice the page follows the system", () => {
  assert.strictEqual(isDarkTheme("system", true), true);
  assert.strictEqual(isDarkTheme("system", false), false);
});

test("toggling picks the opposite of what is showing", () => {
  assert.strictEqual(toggledTheme("dark", false), "light");
  assert.strictEqual(toggledTheme("light", true), "dark");
  assert.strictEqual(toggledTheme("system", true), "light");
  assert.strictEqual(toggledTheme("system", false), "dark");
});
