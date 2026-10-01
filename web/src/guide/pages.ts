/* The MCP page is the repository's own document, read where it lives, so the
   guide and the repository cannot drift apart. */
import mcp from "../../../docs/mcp.md?raw";
import api from "./content/api.md?raw";
import config from "./content/config.md?raw";
import environment from "./content/environment.md?raw";
import example from "./content/example.md?raw";
import index from "./content/index.md?raw";
import otherOptions from "./content/other-options.md?raw";
import v2 from "./content/v2.md?raw";

export type GuidePage = {
  /** The path segment under /guide. The index has none. */
  slug: string;
  /** The name the navigation and the breadcrumb use. */
  title: string;
  /** The page's Markdown. */
  source: string;
};

export type GuideGroup = { label?: string; pages: GuidePage[] };

const overview: GuidePage = { slug: "", title: "Guide", source: index };

export const GUIDE_GROUPS: GuideGroup[] = [
  { pages: [overview] },
  {
    label: "V1 API documentation",
    pages: [
      { slug: "api", title: "Overview", source: api },
      { slug: "environment", title: "Environment", source: environment },
      { slug: "other-options", title: "Other options", source: otherOptions },
      { slug: "example", title: "Example", source: example },
    ],
  },
  {
    label: "More",
    pages: [
      { slug: "v2", title: "V2 proposal", source: v2 },
      { slug: "mcp", title: "MCP", source: mcp },
    ],
  },
];

/* The documentation site never linked its Config page, a leftover of the site
   template. It keeps an address here and stays out of the navigation. */
const unlisted: GuidePage[] = [{ slug: "config", title: "Config", source: config }];

export const GUIDE_PAGES: GuidePage[] = [
  ...GUIDE_GROUPS.flatMap((group) => group.pages),
  ...unlisted,
];

export function guidePath(slug: string): string {
  return slug ? `/guide/${slug}` : "/guide";
}

export function findGuidePage(slug: string | undefined): GuidePage | undefined {
  return GUIDE_PAGES.find((page) => page.slug === (slug ?? ""));
}
