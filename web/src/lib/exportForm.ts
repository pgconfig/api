/** How the generated configuration is rendered. */
export type ExportForm = {
  format: string;
  include_pgbadger: boolean;
  log_format: string;
};

export type Option = { value: string; label: string };

export const FORMAT_OPTIONS: Option[] = [
  { value: "alter_system", label: "ALTER SYSTEM commands" },
  { value: "conf", label: "UNIX-like config file" },
  { value: "stackgres", label: "StackGres-like YAML file" },
];

/** PostgreSQL writes JSON logs from version 15. */
function supportsJsonLog(pgVersion: string): boolean {
  return parseFloat(pgVersion) >= 15;
}

export function logFormatOptions(pgVersion: string): Option[] {
  const options = [
    { value: "stderr", label: "Standard Error output" },
    { value: "csvlog", label: "Comma-separated values" },
    { value: "syslog", label: "Syslog daemon" },
  ];
  if (supportsJsonLog(pgVersion)) {
    options.push({ value: "jsonlog", label: "JSON Log" });
  }
  return options;
}

/**
 * The log format to use after the PostgreSQL version changes. An unset format
 * takes the version's default, and the CSV and JSON formats follow the version
 * across the point where JSON logs became available.
 */
export function nextLogFormat(current: string, pgVersion: string): string {
  const json = supportsJsonLog(pgVersion);
  if (!json && current === "jsonlog") return "csvlog";
  if (json && current === "csvlog") return "jsonlog";
  if (!current) return json ? "jsonlog" : "csvlog";
  return current;
}

/** The export form as API arguments. A switched off option is left out. */
export function buildExportArgs(form: ExportForm): string {
  return Object.entries(form)
    .filter(([, value]) => value !== false)
    .map(([key, value]) => `${key}=${encodeURIComponent(String(value))}`)
    .join("&");
}

export function highlightLanguage(format: string): "sql" | "yaml" | "json" | "ini" {
  switch (format) {
    case "alter_system":
      return "sql";
    case "stackgres":
      return "yaml";
    case "json":
      return "json";
    default:
      return "ini";
  }
}

/** The generated configuration as the text to show and to copy. */
export function formatExportOutput(output: unknown): string {
  if (output == null || output === "") return "";
  if (typeof output === "string") return output;
  return JSON.stringify(output, null, 2);
}
