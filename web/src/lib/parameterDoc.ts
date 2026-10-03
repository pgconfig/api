import { DEFAULT_API_BASE_URL } from "./api.js";

/** The PostgreSQL manual's entry for one parameter, as the server serves it. */
export type ParameterDoc = {
  /** The front matter: type, unit, default, url, and the other settings. */
  fields: Record<string, unknown>;
  /** The manual's text, in Markdown. */
  text: string;
};

/**
 * Where the server that answers the API serves a parameter's documentation:
 * `/parameters/<major version>/<name>.md`.
 */
export function parameterDocUrl(
  baseUrl: string | undefined,
  version: string,
  name: string,
  page: string,
): string {
  const api = new URL(baseUrl || DEFAULT_API_BASE_URL, page);
  const path = `/parameters/${encodeURIComponent(version)}/${encodeURIComponent(name.toLowerCase())}.md`;
  return new URL(path, api).toString();
}

/** Splits the file into its front matter, where every value is JSON, and its text. */
export function parseParameterDoc(markdown: string): ParameterDoc {
  const match = /^---\n([\s\S]*?)\n---\n/.exec(markdown);
  if (!match) {
    return { fields: {}, text: markdown.trim() };
  }
  const fields: Record<string, unknown> = {};
  for (const line of match[1].split("\n")) {
    const separator = line.indexOf(": ");
    if (separator <= 0) continue;
    try {
      fields[line.slice(0, separator)] = JSON.parse(line.slice(separator + 2));
    } catch {
      // A line that is not JSON is left out.
    }
  }
  return { fields, text: markdown.slice(match[0].length).trim() };
}

/** Fetches a parameter's documentation, or `null` when the server has none. */
export async function getParameterDoc(
  url: string,
  signal: AbortSignal,
): Promise<ParameterDoc | null> {
  const response = await fetch(url, { signal, headers: { Accept: "text/markdown" } });
  return response.ok ? parseParameterDoc(await response.text()) : null;
}
