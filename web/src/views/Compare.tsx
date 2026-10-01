import { useEffect, useRef, useState } from "react";
import { Link as RouterLink, useLocation } from "react-router";
import {
  Button,
  PageHeader,
  PageHeaderTitle,
  Pane,
  Spinner,
  Split,
  Splitter,
  useToast,
} from "@momoi-labs/kiso-react";

import { ComparisonTable } from "../components/ComparisonTable.js";
import { ConfigFilters } from "../components/ConfigFilters.js";
import { ExportPanel } from "../components/ExportPanel.js";
import { Icon } from "../components/Icon.js";
import { LoadError } from "../components/LoadError.js";
import {
  EXPORT_PANEL_MAX_SIZE,
  EXPORT_PANEL_MIN_SIZE,
  EXPORT_PANEL_SIZE_COOKIE,
  EXPORT_PANEL_STATE_COOKIE,
  panelCookie,
  readPanelOpen,
  readPanelSize,
} from "../lib/exportPanelState.js";
import type { ConfigForm } from "../lib/formQuery.js";
import { useConfigForm } from "../lib/useConfigForm.js";
import { DESKTOP, useMediaQuery } from "../lib/useMediaQuery.js";
import type { Tuning } from "../lib/useTuning.js";

/**
 * Every profile's recommendation side by side. On a desktop the generated
 * configuration sits in a resizable pane beside the table; narrow screens
 * reach it on the Export page instead.
 */
export function Compare({ form, tuning }: { form: ConfigForm; tuning: Tuning }) {
  const isDesktop = useMediaQuery(DESKTOP);
  const { search } = useLocation();
  const { selectProfile } = useConfigForm();
  const [exportOpen, setExportOpen] = useState(() => readPanelOpen(document.cookie));
  const [size] = useState(() => readPanelSize(document.cookie));
  const pgVersion = String(form.pg_version);

  // Said once when the page opens, as the reminder of what the table compares.
  const notify = useToast();
  const announced = useRef(false);
  useEffect(() => {
    if (announced.current) return;
    announced.current = true;
    notify(
      "neutral",
      `Comparing the ${form.environment_name.toUpperCase()} profile against all profiles.`,
    );
  }, [notify, form.environment_name]);

  function showExport(open: boolean) {
    setExportOpen(open);
    document.cookie = panelCookie(EXPORT_PANEL_STATE_COOKIE, open);
  }

  const table = (
    <ComparisonTable
      comparison={tuning.comparison}
      pgVersion={pgVersion}
      currentEnv={form.environment_name}
      loading={tuning.loading}
      failed={tuning.error !== null}
      compact={!isDesktop}
      onSelectProfile={selectProfile}
    />
  );

  return (
    <div className="page compare-page" id="content">
      <PageHeader
        actions={
          <>
            {tuning.loading && <Spinner label="Loading recommendations" />}
            {isDesktop ? (
              <Button
                size="sm"
                aria-expanded={exportOpen}
                aria-controls="export-panel"
                onClick={() => showExport(!exportOpen)}
              >
                <Icon name="export" />
                Export
              </Button>
            ) : (
              <Button asChild size="sm">
                <RouterLink to={{ pathname: "/export", search }}>
                  <Icon name="export" />
                  Export
                </RouterLink>
              </Button>
            )}
          </>
        }
      >
        <PageHeaderTitle>Profile comparison</PageHeaderTitle>
      </PageHeader>

      <ConfigFilters />

      {tuning.error && <LoadError reason={tuning.error} onRetry={tuning.retry} />}

      {isDesktop ? (
        <Split className="compare-split" data-export={exportOpen ? "open" : "closed"}>
          <Pane className="compare-pane">{table}</Pane>
          <Splitter
            hidden={!exportOpen}
            defaultSize={size}
            min={EXPORT_PANEL_MIN_SIZE}
            max={EXPORT_PANEL_MAX_SIZE}
            aria-label="Resize the comparison and export panes"
            onSizeChange={(next) => {
              document.cookie = panelCookie(EXPORT_PANEL_SIZE_COOKIE, Math.round(next));
            }}
          />
          <Pane
            id="export-panel"
            className="grow export-pane"
            hidden={!exportOpen}
            role="region"
            aria-labelledby="export-panel-title"
          >
            <div className="between">
              <h2 id="export-panel-title" className="t-h3">
                Export
              </h2>
              <Button
                variant="ghost"
                size="sm"
                className="btn-icon"
                aria-label="Close export panel"
                title="Close export panel"
                onClick={() => showExport(false)}
              >
                <Icon name="x" />
              </Button>
            </div>
            <ExportPanel
              layout="split"
              exported={tuning.exported}
              pgVersion={pgVersion}
              onChange={tuning.setExportForm}
            />
          </Pane>
        </Split>
      ) : (
        table
      )}
    </div>
  );
}
