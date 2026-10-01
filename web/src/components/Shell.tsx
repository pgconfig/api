import type { ReactNode } from "react";
import { Link as RouterLink, useLocation, useNavigate } from "react-router";
import {
  ApplicationShell,
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
  Link,
  ThemeSelector,
} from "@momoi-labs/kiso-react";

import { apiVersionLabel } from "../lib/api.js";
import { isGuidePath, type Crumb } from "../lib/routes.js";
import { useTheme } from "../lib/theme.js";
import { GitHubIcon, Icon } from "./Icon.js";

/**
 * The app's frame: brand, the two destinations, the theme, and the breadcrumb.
 * `tuningSearch` is the form's query string, carried by every link back to the
 * comparison so leaving it does not reset the form.
 */
export function Shell({
  apiVersion,
  crumbs,
  tuningSearch,
  children,
}: {
  apiVersion: string | null;
  crumbs: Crumb[];
  tuningSearch: string;
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
                leading: <Icon name="sliders" />,
                onClick: () => navigate(compare),
              },
              {
                href: "/guide",
                label: "Guide",
                active: isGuidePath(pathname),
                leading: <Icon name="book" />,
                onClick: () => navigate("/guide"),
              },
            ],
          },
        ]}
        footer={
          <>
            <Link
              variant="standalone"
              href="https://github.com/momoi-labs/pgconfig"
              target="_blank"
              rel="noreferrer"
            >
              <GitHubIcon />
              Contribute
            </Link>
            <ThemeSelector theme={theme} onChange={setTheme} />
          </>
        }
        header={
          <Breadcrumb>
            <BreadcrumbList>
              {crumbs.map((crumb, index) => (
                <BreadcrumbItems key={crumb.label} crumb={crumb} first={index === 0} />
              ))}
            </BreadcrumbList>
          </Breadcrumb>
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
