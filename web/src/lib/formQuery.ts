/** The server being tuned, as the address bar and the API describe it. */
export type ConfigForm = {
  max_connections: number;
  pg_version: number;
  environment_name: string;
  total_ram: number;
  cpus: number;
  drive_type: string;
  arch: string;
  os_type: string;
};

export const DEFAULT_FORM: ConfigForm = {
  max_connections: 100,
  pg_version: 18,
  environment_name: "WEB",
  total_ram: 4,
  cpus: 2,
  drive_type: "SSD",
  arch: "x86-64",
  os_type: "linux",
};

const NUMBER_PARSERS: Partial<Record<keyof ConfigForm, (value: string) => number>> = {
  max_connections: (value) => parseInt(value, 10),
  pg_version: parseFloat,
  total_ram: (value) => parseInt(value, 10),
  cpus: (value) => parseInt(value, 10),
};

const FIELDS = Object.keys(DEFAULT_FORM) as (keyof ConfigForm)[];

/**
 * Reads the form from a query string. A field the query does not name, or
 * names with a number that cannot be read, keeps its default.
 */
export function parseFormQuery(query: URLSearchParams): ConfigForm {
  const form: Record<string, string | number> = { ...DEFAULT_FORM };
  for (const field of FIELDS) {
    const value = query.get(field);
    if (value === null) continue;
    const parse = NUMBER_PARSERS[field];
    if (!parse) {
      form[field] = value;
      continue;
    }
    const parsed = parse(value);
    if (!Number.isNaN(parsed)) form[field] = parsed;
  }
  return form as ConfigForm;
}

/** The form as API arguments. The API reads memory with its unit. */
export function buildUrlArgs(form: ConfigForm | null | undefined): string {
  if (!form) return "";
  return FIELDS.map((field) => {
    const value = encodeURIComponent(String(form[field]));
    return field === "total_ram" ? `${field}=${value}GB` : `${field}=${value}`;
  }).join("&");
}

/** The form as it is written to the address bar. */
export function formToQuery(form: ConfigForm): Record<string, string> {
  return Object.fromEntries(FIELDS.map((field) => [field, String(form[field])]));
}
