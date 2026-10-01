import { useEffect, useRef, useState } from "react";
import { Button, Pane, Split, Splitter, useToast } from "@momoi-labs/kiso-react";

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
 * Every profile's recommendation side by side, under the row of filters. On a
 * desktop the generated configuration sits in a resizable panel beside the
 * table; narrow screens reach it on the Export page instead.
 */
export function Compare({ form, tuning }: { form: ConfigForm; tuning: Tuning }) {
  const isDesktop = useMediaQuery(DESKTOP);
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
  const error = tuning.error && <LoadError reason={tuning.error} onRetry={tuning.retry} />;

  return (
    <>
      <ConfigFilters />
      <div className="compare-page" id="content">
        {isDesktop ? (
          <Split className="compare-split" data-export={exportOpen ? "open" : "closed"}>
            <Pane className="compare-pane">
              {error}
              {table}
            </Pane>
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
              <div className="export-pane-header">
                <Icon name="file-down" />
                <h2 id="export-panel-title" className="grow truncate">
                  Export
                </h2>
                <Button
                  variant="ghost"
                  size="sm"
                  className="btn-icon"
                  aria-expanded="true"
                  aria-controls="export-panel"
                  aria-label="Close export panel"
                  title="Close export panel"
                  onClick={() => showExport(false)}
                >
                  <Icon name="panel-right-close" />
                </Button>
              </div>
              <div className="export-pane-body">
                <ExportPanel
                  layout="split"
                  exported={tuning.exported}
                  pgVersion={pgVersion}
                  onChange={tuning.setExportForm}
                />
              </div>
            </Pane>
          </Split>
        ) : (
          <div className="compare-stack">
            {error}
            {table}
          </div>
        )}

        {isDesktop && !exportOpen && (
          <button
            type="button"
            className="export-tab"
            aria-controls="export-panel"
            aria-expanded="false"
            title="Open export panel"
            onClick={() => showExport(true)}
          >
            <Icon name="file-down" />
            Export
          </button>
        )}
      </div>
    </>
  );
}
