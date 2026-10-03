import assert from "node:assert/strict";
import { test } from "node:test";
import { parameterDefault } from "../src/lib/parameterDefault.ts";

test("catalog defaults keep units and special values in the comparison", () => {
  for (const [value, unit, format, expected] of [
    ["16384", "8kB", "Byte", "128MB"],
    ["4096", "kB", "Byte", "4MB"],
    ["80", "MB", "Byte", "80MB"],
    ["-1", "8kB", "Byte", "-1"],
    ["2", undefined, "int", "2"],
    ["8", undefined, "int", "8"],
    ["worker", undefined, "string", "worker"],
    ["0.9", undefined, "float32", "0.9"],
    ["16", "8kB", "int", "16"],
    ["", undefined, "string", '""'],
  ]) {
    assert.equal(parameterDefault({ fields: { default: value, unit }, text: "" }, format), expected);
  }
  assert.equal(parameterDefault(null), "");
  assert.equal(parameterDefault({ fields: {}, text: "" }), "");
});
