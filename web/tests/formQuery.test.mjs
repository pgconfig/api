import assert from "node:assert/strict";
import { test } from "node:test";
import { PG_VERSION_OPTIONS } from "../src/lib/options.ts";
import {
  DEFAULT_FORM,
  buildUrlArgs,
  formToQuery,
  parseCount,
  parseFormQuery,
} from "../src/lib/formQuery.ts";

test("an empty query string is the default form", () => {
  assert.deepStrictEqual(parseFormQuery(new URLSearchParams("")), {
    max_connections: 100,
    pg_version: 18,
    environment_name: "WEB",
    total_ram: 4,
    cpus: 2,
    drive_type: "SSD",
    arch: "x86-64",
    os_type: "linux",
  });
});

test("PostgreSQL 19 beta is selectable and shareable while the default stays 18", () => {
  assert.ok(
    PG_VERSION_OPTIONS.some((version) => version.value === "19" && version.label.includes("Beta")),
  );
  assert.strictEqual(DEFAULT_FORM.pg_version, 18);
  const form = { ...DEFAULT_FORM, pg_version: 19 };
  assert.deepStrictEqual(parseFormQuery(new URLSearchParams(formToQuery(form))), form);
  assert.ok(buildUrlArgs(form).includes("pg_version=19&"));
});

test("the query string overrides only the fields it names", () => {
  const form = parseFormQuery(new URLSearchParams("environment_name=OLTP&cpus=8"));
  assert.deepStrictEqual(form, { ...DEFAULT_FORM, environment_name: "OLTP", cpus: 8 });
});

test("numeric fields are read as numbers and versions keep their decimals", () => {
  const form = parseFormQuery(
    new URLSearchParams("max_connections=250&pg_version=9.6&total_ram=16&cpus=4"),
  );
  assert.strictEqual(form.max_connections, 250);
  assert.strictEqual(form.pg_version, 9.6);
  assert.strictEqual(form.total_ram, 16);
  assert.strictEqual(form.cpus, 4);
});

test("a memory value shared with its unit is still read as gigabytes", () => {
  assert.strictEqual(parseFormQuery(new URLSearchParams("total_ram=8GB")).total_ram, 8);
});

test("a number that cannot be read falls back to the default", () => {
  const form = parseFormQuery(new URLSearchParams("cpus=many&pg_version=latest"));
  assert.strictEqual(form.cpus, DEFAULT_FORM.cpus);
  assert.strictEqual(form.pg_version, DEFAULT_FORM.pg_version);
});

test("parameters the form does not know are left out", () => {
  const form = parseFormQuery(new URLSearchParams("utm_source=mail&cpus=4"));
  assert.deepStrictEqual(Object.keys(form), Object.keys(DEFAULT_FORM));
});

test("the API arguments carry every field, with memory in gigabytes", () => {
  assert.strictEqual(
    buildUrlArgs(DEFAULT_FORM),
    "max_connections=100&pg_version=18&environment_name=WEB&total_ram=4GB&cpus=2&drive_type=SSD&arch=x86-64&os_type=linux",
  );
});

test("the API arguments escape values that would break the query string", () => {
  const args = buildUrlArgs({ ...DEFAULT_FORM, environment_name: "A&B=C" });
  assert.ok(args.includes("environment_name=A%26B%3DC"));
});

test("the address bar gets every field as text, without the memory unit", () => {
  assert.deepStrictEqual(formToQuery({ ...DEFAULT_FORM, pg_version: 9.6 }), {
    max_connections: "100",
    pg_version: "9.6",
    environment_name: "WEB",
    total_ram: "4",
    cpus: "2",
    drive_type: "SSD",
    arch: "x86-64",
    os_type: "linux",
  });
});

test("a form survives a round trip through the address bar", () => {
  const form = { ...DEFAULT_FORM, environment_name: "DW", total_ram: 64, pg_version: 9.4 };
  assert.deepStrictEqual(parseFormQuery(new URLSearchParams(formToQuery(form))), form);
});

test("a count is read from what the field holds while it is being typed", () => {
  assert.strictEqual(parseCount("8"), 8);
  assert.strictEqual(parseCount("128"), 128);
  // A fraction is cut to its whole part, as the form always did.
  assert.strictEqual(parseCount("1.5"), 1);
});

test("a field that is empty or below one has no count yet", () => {
  for (const text of ["", " ", "0", "-5", "abc"]) {
    assert.strictEqual(parseCount(text), null, JSON.stringify(text));
  }
});
