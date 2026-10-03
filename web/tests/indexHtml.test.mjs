import assert from "node:assert/strict";
import { test } from "node:test";
import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const html = await readFile(join(root, "index.html"), "utf8");

test("the document carries Kiso's appearance and leaves the theme to the system", () => {
  const tag = html.match(/<html([^>]*)>/)[1];
  const attributes = Object.fromEntries(
    [...tag.matchAll(/([a-z-]+)="([^"]*)"/g)].map((match) => [match[1], match[2]]),
  );
  assert.deepStrictEqual(attributes, {
    lang: "en",
    "data-accent": "cobalt",
    "data-border-style": "solid",
    "data-corner-style": "square",
    "data-corner-marks": "none",
    "data-corner-size": "off",
    "data-mark-size": "large",
    "data-visual-style": "default",
    "data-app-shell": "default",
  });
});

test("a stored theme is applied before the app loads", () => {
  const themeScript = html.indexOf('localStorage.getItem("theme")');
  const appScript = html.indexOf('src="/src/main.tsx"');
  assert.ok(themeScript > 0, "no theme script");
  assert.ok(themeScript < appScript, "the theme script runs after the app");
});
