/** The tuning endpoints live on the origin that serves the app. */
export const DEFAULT_API_BASE_URL = "/v1/tuning/";

const TIMEOUT_MS = 30000;

export type TuningResponse = {
  /** The `data` of a JSON answer, or the text of a rendered configuration. */
  output: unknown;
  /** The API version that answered, when the answer carries one. */
  version: string | null;
};

type RequestOptions = {
  baseUrl?: string;
  signal?: AbortSignal;
  fetcher?: typeof fetch;
};

export function apiUrl(baseUrl: string | undefined, path: string, query: string): string {
  const base = (baseUrl || DEFAULT_API_BASE_URL).replace(/\/+$/, "");
  return `${base}/${path}?${query}`;
}

/**
 * The API answers JSON for data and plain text for a rendered configuration
 * file, so a body that is not a JSON object is the output itself.
 */
export function decodeBody(body: string): TuningResponse {
  let parsed: unknown;
  try {
    parsed = JSON.parse(body);
  } catch {
    return { output: body, version: null };
  }
  if (!parsed || typeof parsed !== "object") {
    return { output: body, version: null };
  }
  const { data, meta } = parsed as { data?: unknown; meta?: { version?: unknown } };
  const version = typeof meta?.version === "string" && meta.version ? meta.version : null;
  return { output: data, version };
}

function errorMessage(status: number, body: string): string {
  const message = `Request failed with status code ${status}`;
  try {
    const detail = (JSON.parse(body) as { errors?: { message?: unknown } }).errors?.message;
    return typeof detail === "string" && detail ? `${message}: ${detail}` : message;
  } catch {
    return message;
  }
}

/** Calls one tuning endpoint with an already encoded query string. */
export async function getTuning(
  path: string,
  query: string,
  { baseUrl, signal, fetcher = fetch }: RequestOptions = {},
): Promise<TuningResponse> {
  const timeout = AbortSignal.timeout(TIMEOUT_MS);
  const response = await fetcher(apiUrl(baseUrl, path, query), {
    signal: signal ? AbortSignal.any([signal, timeout]) : timeout,
  });
  const body = await response.text();
  if (!response.ok) throw new Error(errorMessage(response.status, body));
  return decodeBody(body);
}

/** The API version as the sidebar shows it, without the build suffix. */
export function apiVersionLabel(raw: string | null | undefined): string | null {
  if (!raw) return null;
  const semver = String(raw).split(" (")[0].trim();
  return `api (${semver})`;
}
