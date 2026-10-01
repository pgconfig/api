import assert from "node:assert/strict";
import { test } from "node:test";
import {
  EXPORT_PANEL_DEFAULT_SIZE,
  EXPORT_PANEL_MAX_SIZE,
  EXPORT_PANEL_MIN_SIZE,
  panelCookie,
  readPanelOpen,
  readPanelSize,
} from "../src/lib/exportPanelState.ts";

test("the export panel starts open", () => {
  assert.strictEqual(readPanelOpen(""), true);
  assert.strictEqual(readPanelOpen("theme=dark"), true);
});

test("a closed export panel stays closed on the next visit", () => {
  assert.strictEqual(readPanelOpen("export_panel_state=false"), false);
  assert.strictEqual(readPanelOpen("a=1; export_panel_state=false; b=2"), false);
});

test("a reopened export panel stays open on the next visit", () => {
  assert.strictEqual(readPanelOpen("export_panel_state=true"), true);
});

test("the split starts at its default size", () => {
  assert.strictEqual(readPanelSize(""), EXPORT_PANEL_DEFAULT_SIZE);
  assert.strictEqual(readPanelSize("export_panel_size=wide"), EXPORT_PANEL_DEFAULT_SIZE);
});

test("a resized split keeps its size on the next visit", () => {
  assert.strictEqual(readPanelSize("a=1; export_panel_size=62; b=2"), 62);
});

test("a stored size outside the allowed range is brought back into it", () => {
  assert.strictEqual(readPanelSize("export_panel_size=5"), EXPORT_PANEL_MIN_SIZE);
  assert.strictEqual(readPanelSize("export_panel_size=99"), EXPORT_PANEL_MAX_SIZE);
});

test("the pixel width the previous app stored is not read as a size", () => {
  assert.strictEqual(readPanelSize("export_panel_width=352"), EXPORT_PANEL_DEFAULT_SIZE);
});

test("panel preferences are remembered site-wide for a week", () => {
  assert.strictEqual(
    panelCookie("export_panel_state", false),
    "export_panel_state=false; path=/; max-age=604800",
  );
});
