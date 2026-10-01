import assert from "node:assert/strict";
import { test } from "node:test";
import { highlightCode, renderMarkdown } from "../src/lib/markdown.ts";

test("headings can be linked to and follow the page hierarchy", () => {
  const html = renderMarkdown("# API Specification\n\n## How it works\n\n### `conf` format");
  assert.ok(html.includes('<h1 id="api-specification" class="t-h1">API Specification</h1>'));
  assert.ok(html.includes('<h2 id="how-it-works" class="t-h2">How it works</h2>'));
  assert.ok(html.includes('<h3 id="conf-format" class="t-h3"><code>conf</code> format</h3>'));
});

test("two headings with the same text get different anchors", () => {
  const html = renderMarkdown("## Errors\n\n## Errors");
  assert.ok(html.includes('id="errors"'));
  assert.ok(html.includes('id="errors-1"'));
});

test("a code block in a known language is highlighted", () => {
  const html = renderMarkdown('```json\n{ "name": "pgconfig" }\n```');
  assert.ok(html.includes('<pre><code class="hljs language-json">'));
  assert.ok(html.includes('<span class="hljs-attr">&quot;name&quot;</span>'));
});

test("a code block in an unknown language is shown as written, never run", () => {
  const html = renderMarkdown("```text\n<script>alert(1)</script>\n```");
  assert.ok(html.includes("&lt;script&gt;alert(1)&lt;/script&gt;"));
  assert.ok(!html.includes("<script>"));
});

test("a table scrolls inside its own frame", () => {
  const html = renderMarkdown("| Input | Required |\n| --- | --- |\n| `total_ram` | Yes |");
  assert.ok(
    html.includes('<div class="table-wrap"><div class="table-scroll"><table class="table">'),
  );
  assert.ok(html.includes("</table></div></div>"));
});

test("a table written as HTML gets the same frame", () => {
  const html = renderMarkdown(
    '<table class="table table-bordered"><thead><tr><th>Name</th></tr></thead></table>',
  );
  assert.ok(
    html.includes('<div class="table-wrap"><div class="table-scroll"><table class="table">'),
  );
  assert.ok(!html.includes("table-bordered"));
});

test("a link to another site opens in a new tab", () => {
  const html = renderMarkdown("[pgconfig.org](https://pgconfig.org)");
  assert.ok(
    html.includes('<a href="https://pgconfig.org" target="_blank" rel="noreferrer">pgconfig.org</a>'),
  );
});

test("a link inside the app stays in the same tab", () => {
  const html = renderMarkdown("[API](/guide/api)");
  assert.ok(html.includes('<a href="/guide/api">API</a>'));
});

test("single line breaks are kept only when asked for", () => {
  assert.ok(renderMarkdown("one\ntwo", { breaks: true }).includes("one<br>two"));
  assert.ok(!renderMarkdown("one\ntwo").includes("<br>"));
});

test("nothing to render is an empty string", () => {
  assert.strictEqual(renderMarkdown(""), "");
});

test("generated configuration is highlighted in the language of its format", () => {
  assert.ok(highlightCode("shared_buffers = 1GB", "ini").includes('<span class="hljs-attr">'));
  assert.ok(
    highlightCode("ALTER SYSTEM SET work_mem TO '4MB';", "sql").includes('<span class="hljs-keyword">'),
  );
  assert.ok(highlightCode("kind: SGPostgresConfig", "yaml").includes('<span class="hljs-attr">'));
});
