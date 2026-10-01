import { Fragment, useId, useMemo, useState } from "react";
import {
  Skeleton,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@momoi-labs/kiso-react";

import {
  formatConfigs,
  type ComparisonParam,
  type EnvironmentConfig,
} from "../lib/formatters.js";
import { ENV_COLUMN_TO_PROFILE, profileColumnLabel } from "../lib/options.js";
import { ComparisonRowDetail } from "./ComparisonRowDetail.js";
import { Icon } from "./Icon.js";

const ALL_ENVS = Object.keys(ENV_COLUMN_TO_PROFILE);

function isSelected(env: string, currentEnv: string): boolean {
  return env.toUpperCase() === currentEnv.toUpperCase();
}

function envKeyForProfile(currentEnv: string): string {
  return ALL_ENVS.find((env) => isSelected(env, currentEnv)) ?? "web";
}

/** A parameter name that may wrap after an underscore instead of mid-word. */
function breakable(name: string) {
  return name.split("_").map((part, index) => (
    <Fragment key={index}>
      {index > 0 && (
        <>
          _<wbr />
        </>
      )}
      {part}
    </Fragment>
  ));
}

type CategoryProps = {
  title: string;
  params: ComparisonParam[];
  currentEnv: string;
  pgVersion: string;
  /** Narrow screens keep only the default and the selected profile. */
  compact: boolean;
  onSelectProfile: (environmentName: string) => void;
};

function ComparisonCategory({
  title,
  params,
  currentEnv,
  pgVersion,
  compact,
  onSelectProfile,
}: CategoryProps) {
  const [expanded, setExpanded] = useState<ReadonlySet<string>>(new Set());
  const id = useId();
  const envs = compact ? [envKeyForProfile(currentEnv)] : ALL_ENVS;

  function toggle(name: string) {
    const next = new Set(expanded);
    if (next.has(name)) next.delete(name);
    else next.add(name);
    setExpanded(next);
  }

  return (
    <section className="stack-sm" aria-labelledby={`${id}-title`}>
      <h2 id={`${id}-title`} className="t-h3">
        {title}
      </h2>
      {compact && (
        <p className="muted t-label">
          Comparing PostgreSQL defaults with recommended {profileColumnLabel(currentEnv)}{" "}
          values. Change application profile above to switch environments.
        </p>
      )}
      <div className="table-wrap">
        <Table
          className="comparison-table"
          data-compact={compact || undefined}
          aria-labelledby={`${id}-title`}
        >
          <colgroup>
            <col className="comparison-col-name" />
            <col className="comparison-col-default" />
            {envs.map((env) => (
              <col key={env} />
            ))}
          </colgroup>
          <TableHeader>
            <TableRow>
              <TableHead>Parameter</TableHead>
              <TableHead>{compact ? "Default" : "Default value"}</TableHead>
              {envs.map((env) => {
                const selected = isSelected(env, currentEnv);
                return (
                  <TableHead key={env} data-selected={selected || undefined}>
                    {compact ? (
                      profileColumnLabel(currentEnv)
                    ) : (
                      <button
                        type="button"
                        className="comparison-profile"
                        aria-pressed={selected}
                        title={`Select the ${env.toUpperCase()} profile`}
                        onClick={() => onSelectProfile(ENV_COLUMN_TO_PROFILE[env])}
                      >
                        {env.toUpperCase()}
                      </button>
                    )}
                  </TableHead>
                );
              })}
            </TableRow>
          </TableHeader>
          <TableBody>
            {params.map((param) => {
              const open = expanded.has(param.name);
              const detailId = `${id}-${param.name}`;
              const defaultValue = String(param.documentation?.default_value ?? "");
              return (
                <Fragment key={param.name}>
                  {/* The button is the control; the row only widens its target. */}
                  <TableRow
                    className="comparison-row"
                    data-expanded={open || undefined}
                    onClick={() => toggle(param.name)}
                  >
                    <TableCell>
                      <button
                        type="button"
                        className="comparison-toggle"
                        aria-expanded={open}
                        aria-controls={open ? detailId : undefined}
                      >
                        <Icon name={open ? "arrow-down-s-line" : "arrow-right-s-line"} />
                        <span className="mono">{breakable(param.name)}</span>
                      </button>
                    </TableCell>
                    <TableCell className="mono muted" title={defaultValue}>
                      {defaultValue}
                    </TableCell>
                    {envs.map((env) => {
                      const value = String(param[env] ?? "");
                      return (
                        <TableCell
                          key={env}
                          className="mono"
                          title={value}
                          data-selected={isSelected(env, currentEnv) || undefined}
                          data-pick={compact ? undefined : true}
                          onClick={
                            compact
                              ? undefined
                              : (event) => {
                                  event.stopPropagation();
                                  onSelectProfile(ENV_COLUMN_TO_PROFILE[env]);
                                }
                          }
                        >
                          {value}
                        </TableCell>
                      );
                    })}
                  </TableRow>
                  {open && (
                    <TableRow id={detailId} className="comparison-detail-row">
                      <TableCell colSpan={envs.length + 2}>
                        <ComparisonRowDetail row={param} pgVersion={pgVersion} />
                      </TableCell>
                    </TableRow>
                  )}
                </Fragment>
              );
            })}
          </TableBody>
        </Table>
      </div>
    </section>
  );
}

/** The table's shape while the first answer is on its way. */
function ComparisonSkeleton({ compact }: { compact: boolean }) {
  const columns = compact ? 3 : ALL_ENVS.length + 2;
  return (
    <div className="stack" aria-busy="true">
      <p className="visually-hidden" role="status">
        Loading recommendations…
      </p>
      {[4, 4].map((rows, section) => (
        <div key={section} className="stack-sm">
          <Skeleton className="comparison-skeleton-title" />
          <div className="table-wrap">
            <Table className="comparison-table" aria-hidden="true">
              <TableBody>
                {Array.from({ length: rows }, (_, row) => (
                  <TableRow key={row}>
                    {Array.from({ length: columns }, (_, column) => (
                      <TableCell key={column}>
                        <Skeleton />
                      </TableCell>
                    ))}
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        </div>
      ))}
    </div>
  );
}

export function ComparisonTable({
  comparison,
  pgVersion,
  currentEnv,
  loading,
  failed,
  compact,
  onSelectProfile,
}: {
  comparison: EnvironmentConfig[];
  pgVersion: string;
  currentEnv: string;
  loading: boolean;
  /** The request failed; the page says why, so no placeholder stands in. */
  failed: boolean;
  compact: boolean;
  onSelectProfile: (environmentName: string) => void;
}) {
  const categories = useMemo(() => formatConfigs(comparison), [comparison]);

  if (categories.length === 0) {
    return failed && !loading ? null : <ComparisonSkeleton compact={compact} />;
  }

  return (
    <div className="stack comparison-tables" aria-busy={loading}>
      {categories.map((category) => (
        <ComparisonCategory
          key={category.name}
          title={category.name}
          params={category.params}
          currentEnv={currentEnv}
          pgVersion={pgVersion}
          compact={compact}
          onSelectProfile={onSelectProfile}
        />
      ))}
    </div>
  );
}
