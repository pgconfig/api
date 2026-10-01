/**
 * Tests for GitHub-style alerts markdown parsing
 */

import assert from "node:assert/strict";
import { describe, test } from "node:test";
import { marked } from "marked";
import { parseMarkdownBlocks } from "../src/lib/parseMarkdownBlocks.ts";

describe("parseMarkdownBlocks", () => {
  test("renders NOTE alert block", () => {
    const markdown = `> [!NOTE]
> This is a note`

    const blocks = parseMarkdownBlocks(markdown)
    assert.strictEqual(blocks.length, 1)
    assert.partialDeepStrictEqual(blocks[0], {
      type: "alert",
      alertType: "NOTE",
      content: "This is a note",
    })
  })

  test("renders WARNING alert block", () => {
    const markdown = `> [!WARNING]
> This is a warning`

    const blocks = parseMarkdownBlocks(markdown)
    assert.partialDeepStrictEqual(blocks[0], {
      type: "alert",
      alertType: "WARNING",
    })
  })

  test("handles prose before and after alerts", () => {
    const markdown = `Intro paragraph.

> [!TIP]
> Helpful advice here

Closing paragraph.`

    const blocks = parseMarkdownBlocks(markdown)
    assert.strictEqual(blocks.length, 3)
    assert.strictEqual(blocks[0].type, "markdown")
    assert.strictEqual(blocks[1].type, "alert")
    assert.strictEqual(blocks[2].type, "markdown")
  })

  test("handles multiple consecutive alerts", () => {
    const markdown = `> [!WARNING]
> Warning body

> [!NOTE]
> Note body`

    const blocks = parseMarkdownBlocks(markdown)
    assert.strictEqual(blocks.length, 2)
    assert.strictEqual(blocks[0].alertType, "WARNING")
    assert.strictEqual(blocks[1].alertType, "NOTE")
  })

  test("handles case-insensitive alert types", () => {
    const markdown = `> [!note]
> Lowercase note`

    const blocks = parseMarkdownBlocks(markdown)
    assert.strictEqual(blocks[0].alertType, "NOTE")
  })

  test("alert body supports markdown rendering", () => {
    const markdown = `> [!WARNING]
> This is **bold** and *italic*`

    const html = marked(parseMarkdownBlocks(markdown)[0].content)
    assert.ok(html.includes("<strong>bold</strong>"))
    assert.ok(html.includes("<em>italic</em>"))
  })

  test("handles code blocks inside alerts", () => {
    const markdown = `> [!TIP]
> Example formula:
> \`\`\`
> max = work_mem × operations
> \`\`\``

    const blocks = parseMarkdownBlocks(markdown)
    const html = marked(blocks[0].content)
    assert.ok(html.includes("<code>"))
    assert.ok(html.includes("max = work_mem"))
  })

  test("handles tables inside alerts", () => {
    const markdown = `> [!TIP]
> Connection recommendations:
>
> | Scenario | Connections |
> |----------|------------|
> | With pooling | 20-50 |`

    const blocks = parseMarkdownBlocks(markdown)
    const html = marked(blocks[0].content)
    assert.ok(html.includes("<table>"))
    assert.ok(html.includes("With pooling"))
  })
})
