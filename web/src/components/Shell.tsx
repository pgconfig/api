import type { ReactNode } from "react";
import { Link as RouterLink, useLocation, useNavigate } from "react-router";
import {
  ApplicationShell,
  Badge,
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
  Dot,
  Separator,
  ThemeSelector,
} from "@momoi-labs/kiso-react";

import { apiVersionLabel } from "../lib/api.js";
import { isGuidePath, isMcpPath, type Crumb } from "../lib/routes.js";
import { useTheme } from "../lib/theme.js";
import { Icon } from "./Icon.js";

const REPOSITORY = "https://github.com/momoi-labs/pgconfig";

/**
 * The app's frame. The sidebar has the brand, the four destinations, and the
 * theme at its foot. The header has Kiso's sidebar toggle, the page's name, and
 * whatever `actions` the page puts beside it. `tuningSearch` is the form's
 * query string, carried by every link back to the comparison so leaving it
 * does not reset the form.
 */
export function Shell({
  apiVersion,
  crumbs,
  tuningSearch,
  actions,
  children,
}: {
  apiVersion: string | null;
  crumbs: Crumb[];
  tuningSearch: string;
  actions?: ReactNode;
  children: ReactNode;
}) {
  const [theme, setTheme] = useTheme();
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const versionLabel = apiVersionLabel(apiVersion);
  const compare = { pathname: "/", search: tuningSearch };

  return (
    <>
      <a className="skip-link" href="#content">
        Skip to content
      </a>
      <ApplicationShell
        collapsible
        togglePlacement="header"
        brand={
          <div className="brand-block">
            <RouterLink to={compare} className="brand-logo" aria-label="PGConfig home">
              <img src="/pgconfig.svg" alt="" width="1025" height="904" />
            </RouterLink>
            {versionLabel && (
              <p className="t-metadata muted mono truncate" title={versionLabel}>
                {versionLabel}
              </p>
            )}
            <p className="brand-wordmark mono" aria-label="pgconfig">
              <span className="muted">pg</span>
              <span>config</span>
              <span className="muted">=#</span>
              <span className="terminal-cursor" aria-hidden="true" />
            </p>
          </div>
        }
        navigation={[
          {
            destinations: [
              {
                href: `/${tuningSearch}`,
                label: "Profile comparison",
                active: pathname === "/" || pathname === "/tuning",
                leading: <Icon name="equalizer-line" />,
                onClick: () => navigate(compare),
              },
              {
                href: "/guide",
                label: "Docs",
                active: isGuidePath(pathname) && !isMcpPath(pathname),
                leading: <Icon name="book-open-line" />,
                onClick: () => navigate("/guide"),
              },
              {
                // The newest way in, so it gets a place of its own and a mark.
                href: "/guide/mcp",
                label: "MCP",
                active: isMcpPath(pathname),
                leading: <Icon name="robot-2-line" />,
                trailing: (
                  <Badge variant="info">
                    <Dot variant="info" />
                    New
                  </Badge>
                ),
                onClick: () => navigate("/guide/mcp"),
              },
              {
                href: REPOSITORY,
                label: "Contribute",
                leading: <Icon name="github-fill" />,
                // The repository is another site, so it opens beside the app.
                onClick: () => window.open(REPOSITORY, "_blank", "noopener,noreferrer"),
              },
            ],
          },
        ]}
        footer={<ThemeSelector theme={theme} onChange={setTheme} />}
        header={
          <>
            <Separator orientation="vertical" className="header-separator" />
            <Breadcrumb className="grow header-title">
              <BreadcrumbList>
                {crumbs.map((crumb, index) => (
                  <BreadcrumbItems key={crumb.label} crumb={crumb} first={index === 0} />
                ))}
              </BreadcrumbList>
            </Breadcrumb>
            {actions && <div className="header-actions">{actions}</div>}
          </>
        }
      >
        {children}
      </ApplicationShell>
    </>
  );
}

function BreadcrumbItems({ crumb, first }: { crumb: Crumb; first: boolean }) {
  return (
    <>
      {!first && <BreadcrumbSeparator />}
      <BreadcrumbItem>
        {crumb.to ? (
          <BreadcrumbLink asChild>
            <RouterLink to={crumb.to}>{crumb.label}</RouterLink>
          </BreadcrumbLink>
        ) : (
          <BreadcrumbPage>{crumb.label}</BreadcrumbPage>
        )}
      </BreadcrumbItem>
    </>
  );
}
