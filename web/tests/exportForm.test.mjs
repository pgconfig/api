import assert from "node:assert/strict";
import { test } from "node:test";
import {
  buildExportArgs,
  formatExportOutput,
  highlightLanguage,
  logFormatOptions,
  nextLogFormat,
} from "../src/lib/exportForm.ts";

test("a new export form logs as JSON from PostgreSQL 15, and as CSV before it", () => {
  assert.strictEqual(nextLogFormat("", "18"), "jsonlog");
  assert.strictEqual(nextLogFormat("", "15"), "jsonlog");
  assert.strictEqual(nextLogFormat("", "14"), "csvlog");
  assert.strictEqual(nextLogFormat("", "9.6"), "csvlog");
});

test("moving to a version without JSON logs falls back to CSV", () => {
  assert.strictEqual(nextLogFormat("jsonlog", "13"), "csvlog");
});

test("moving to a version with JSON logs upgrades a CSV choice", () => {
  assert.strictEqual(nextLogFormat("csvlog", "16"), "jsonlog");
});

test("a log format every version supports is kept across versions", () => {
  assert.strictEqual(nextLogFormat("stderr", "18"), "stderr");
  assert.strictEqual(nextLogFormat("syslog", "12"), "syslog");
  assert.strictEqual(nextLogFormat("csvlog", "12"), "csvlog");
  assert.strictEqual(nextLogFormat("jsonlog", "17"), "jsonlog");
});

test("JSON logs are offered only from PostgreSQL 15", () => {
  const values = (version) => logFormatOptions(version).map((option) => option.value);
  assert.deepStrictEqual(values("14"), ["stderr", "csvlog", "syslog"]);
  assert.deepStrictEqual(values("15"), ["stderr", "csvlog", "syslog", "jsonlog"]);
});

test("the export arguments carry the format, pgBadger and the log format", () => {
  assert.strictEqual(
    buildExportArgs({ format: "conf", include_pgbadger: true, log_format: "jsonlog" }),
    "format=conf&include_pgbadger=true&log_format=jsonlog",
  );
});

test("a switched off option is left out of the export arguments", () => {
  assert.strictEqual(
    buildExportArgs({ format: "alter_system", include_pgbadger: false, log_format: "csvlog" }),
    "format=alter_system&log_format=csvlog",
  );
});

test("each export format is highlighted in its own language", () => {
  assert.strictEqual(highlightLanguage("alter_system"), "sql");
  assert.strictEqual(highlightLanguage("stackgres"), "yaml");
  assert.strictEqual(highlightLanguage("json"), "json");
  assert.strictEqual(highlightLanguage("conf"), "ini");
});

test("the generated configuration is shown as text", () => {
  assert.strictEqual(formatExportOutput("shared_buffers = 1GB\n"), "shared_buffers = 1GB\n");
  assert.strictEqual(formatExportOutput({ a: 1 }), '{\n  "a": 1\n}');
});

test("no output yet is shown as nothing", () => {
  assert.strictEqual(formatExportOutput(null), "");
  assert.strictEqual(formatExportOutput(undefined), "");
  assert.strictEqual(formatExportOutput(""), "");
});
