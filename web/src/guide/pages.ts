/* The MCP page is the repository's own document, read where it lives, so the
   guide and the repository cannot drift apart. */
import mcp from "../../../docs/mcp.md?raw";
import api from "./content/api.md?raw";
import environment from "./content/environment.md?raw";
import example from "./content/example.md?raw";
import index from "./content/index.md?raw";
import otherOptions from "./content/other-options.md?raw";

export type GuidePage = {
  /** The path segment under /guide. The index has none. */
  slug: string;
  /** The name the navigation and the breadcrumb use. */
  title: string;
  /** The page's Markdown. */
  source: string;
};

export type GuideGroup = { label?: string; pages: GuidePage[] };

const introduction: GuidePage = { slug: "", title: "Introduction", source: index };

export const GUIDE_GROUPS: GuideGroup[] = [
  { pages: [introduction] },
  {
    label: "REST v1",
    pages: [
      { slug: "api", title: "Get a configuration", source: api },
      { slug: "environment", title: "Profiles", source: environment },
      { slug: "other-options", title: "Other endpoints", source: otherOptions },
      { slug: "example", title: "Example and rules", source: example },
    ],
  },
  {
    label: "AI agents",
    pages: [{ slug: "mcp", title: "MCP", source: mcp }],
  },
];

export const GUIDE_PAGES: GuidePage[] = GUIDE_GROUPS.flatMap((group) => group.pages);

export function guidePath(slug: string): string {
  return slug ? `/guide/${slug}` : "/guide";
}

export function findGuidePage(slug: string | undefined): GuidePage | undefined {
  return GUIDE_PAGES.find((page) => page.slug === (slug ?? ""));
}
