import { useEffect, useMemo, useState } from "react";
import { Button, EmptyState, EmptyStateDescription, Label, Switch, useToast } from "@momoi-labs/kiso-react";

import {
  FORMAT_OPTIONS,
  formatExportOutput,
  highlightLanguage,
  logFormatOptions,
  nextLogFormat,
  type ExportForm,
} from "../lib/exportForm.js";
import { highlightCode } from "../lib/markdown.js";
import { Icon } from "./Icon.js";
import { SelectField } from "./SelectField.js";

/**
 * The generated configuration and the options that shape it. `split` is the
 * panel beside the comparison, which fills its pane; `page` is the page of its
 * own that narrow screens use.
 */
export function ExportPanel({
  exported,
  pgVersion,
  layout,
  onChange,
}: {
  exported: unknown;
  pgVersion: string;
  layout: "page" | "split";
  onChange: (form: ExportForm) => void;
}) {
  const notify = useToast();
  const [form, setForm] = useState<ExportForm>(() => ({
    format: "conf",
    include_pgbadger: true,
    log_format: nextLogFormat("", pgVersion),
  }));

  // The log formats on offer depend on the PostgreSQL version.
  useEffect(() => {
    setForm((current) => {
      const log_format = nextLogFormat(current.log_format, pgVersion);
      return log_format === current.log_format ? current : { ...current, log_format };
    });
  }, [pgVersion]);

  useEffect(() => {
    onChange(form);
  }, [form, onChange]);

  const text = formatExportOutput(exported);
  const language = highlightLanguage(form.format);
  const html = useMemo(() => highlightCode(text, language), [text, language]);

  async function copy() {
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
      notify("success", "Copied to clipboard");
    } catch (cause) {
      notify("error", "Failed to copy", String(cause));
    }
  }

  return (
    <div className="export-panel" data-layout={layout}>
      <div className="export-options">
        <SelectField
          label="Export format"
          placeholder="Select format"
          value={form.format}
          options={FORMAT_OPTIONS}
          onChange={(format) => setForm((current) => ({ ...current, format }))}
        />
        <SelectField
          label="Log format"
          placeholder="Select log format"
          value={form.log_format}
          options={logFormatOptions(pgVersion)}
          disabled={!form.include_pgbadger}
          onChange={(log_format) => setForm((current) => ({ ...current, log_format }))}
        />
        <div className="row export-pgbadger">
          <Switch
            id="include-pgbadger"
            checked={form.include_pgbadger}
            onCheckedChange={(include_pgbadger) =>
              setForm((current) => ({ ...current, include_pgbadger }))
            }
          />
          <Label htmlFor="include-pgbadger">Include PGBadger log configuration</Label>
        </div>
      </div>

      {/* One frame for the output: its title and the copy button, then the text.
          The code block inside gives up its own frame through Kiso's border
          style, so the card is the only one. */}
      <section className="card export-output">
        <div className="between export-output-header">
          <span className="t-label muted">Generated configuration</span>
          <Button size="sm" disabled={!text} onClick={copy}>
            <Icon name="file-copy-line" />
            Copy
          </Button>
        </div>

        {text ? (
          <div className="export-code" data-border-style="none">
            <pre>
              <code className={`hljs ${language}`} dangerouslySetInnerHTML={{ __html: html }} />
            </pre>
          </div>
        ) : (
          <EmptyState size="sm" variant="informational">
            <EmptyStateDescription>Configuration output will appear here.</EmptyStateDescription>
          </EmptyState>
        )}
      </section>
    </div>
  );
}
