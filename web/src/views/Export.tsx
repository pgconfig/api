import { useEffect } from "react";
import { Link as RouterLink, useLocation, useNavigate } from "react-router";
import { Button, PageHeader, PageHeaderTitle, Spinner } from "@momoi-labs/kiso-react";

import { ConfigFilters } from "../components/ConfigFilters.js";
import { ExportPanel } from "../components/ExportPanel.js";
import { LoadError } from "../components/LoadError.js";
import { EXPORT_PANEL_STATE_COOKIE, panelCookie } from "../lib/exportPanelState.js";
import type { ConfigForm } from "../lib/formQuery.js";
import { DESKTOP, useMediaQuery } from "../lib/useMediaQuery.js";
import type { Tuning } from "../lib/useTuning.js";

/**
 * The generated configuration on a page of its own, for narrow screens. A
 * desktop shows it beside the comparison, so it is sent there with the panel
 * open.
 */
export function Export({ form, tuning }: { form: ConfigForm; tuning: Tuning }) {
  const isDesktop = useMediaQuery(DESKTOP);
  const { search } = useLocation();
  const navigate = useNavigate();

  useEffect(() => {
    if (!isDesktop) return;
    document.cookie = panelCookie(EXPORT_PANEL_STATE_COOKIE, true);
    navigate({ pathname: "/", search }, { replace: true });
  }, [isDesktop, navigate, search]);

  if (isDesktop) return null;

  return (
    <div className="page" id="content">
      <PageHeader
        actions={
          <>
            {tuning.loading && <Spinner label="Loading configuration" />}
            <Button asChild size="sm">
              <RouterLink to={{ pathname: "/", search }}>Compare</RouterLink>
            </Button>
          </>
        }
      >
        <PageHeaderTitle>Export</PageHeaderTitle>
      </PageHeader>

      <ConfigFilters />

      {tuning.error && <LoadError reason={tuning.error} onRetry={tuning.retry} />}

      <ExportPanel
        layout="page"
        exported={tuning.exported}
        pgVersion={String(form.pg_version)}
        onChange={tuning.setExportForm}
      />
    </div>
  );
}
