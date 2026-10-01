/** A step of the breadcrumb. The last one is the current page and has no link. */
export type Crumb = { label: string; to?: string };

/** The comparison, its previous address, and the export page. */
const TUNING_PATHS = ["/", "/tuning", "/export"];

/** The router ignores a trailing slash, so the helpers below do too. */
function normalize(pathname: string): string {
  return pathname.replace(/\/+$/, "") || "/";
}

/** Whether the page at this address tunes a server, and so talks to the API. */
export function isTuningPath(pathname: string): boolean {
  return TUNING_PATHS.includes(normalize(pathname));
}

export function isGuidePath(pathname: string): boolean {
  const path = normalize(pathname);
  return path === "/guide" || path.startsWith("/guide/");
}

/** The guide page an address names. The index has an empty slug. */
export function guideSlug(pathname: string): string {
  return normalize(pathname).slice("/guide/".length);
}

/**
 * The breadcrumb for an address. A page of the app is named alone, as the
 * header's title. `guideTitle` gives the title of a docs page, or
 * nothing for an unknown one.
 */
export function crumbsFor(
  pathname: string,
  guideTitle: (slug: string) => string | undefined,
): Crumb[] {
  const path = normalize(pathname);
  if (path === "/" || path === "/tuning") return [{ label: "Profile comparison" }];
  if (path === "/export") return [{ label: "Export" }];
  if (!isGuidePath(path)) return [{ label: "Not found" }];

  const slug = guideSlug(path);
  if (!slug) return [{ label: "Docs" }];
  return [{ label: "Docs", to: "/guide" }, { label: guideTitle(slug) ?? "Not found" }];
}
