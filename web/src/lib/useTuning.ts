import { useCallback, useEffect, useState } from "react";

import { getTuning, type TuningResponse } from "./api.js";
import { buildExportArgs, type ExportForm } from "./exportForm.js";
import type { EnvironmentConfig } from "./formatters.js";

export type Tuning = {
  /** Every profile's recommendation for the current form. */
  comparison: EnvironmentConfig[];
  /** The selected profile's configuration in the chosen export format. */
  exported: unknown;
  apiVersion: string | null;
  loading: boolean;
  /** Why the last request failed, until a later one succeeds. */
  error: string | null;
  setExportForm: (form: ExportForm) => void;
  retry: () => void;
};

const baseUrl = import.meta.env.VITE_API_BASE_URL as string | undefined;

/**
 * Keeps the comparison and the exported configuration in step with the form.
 * `args` is the form as API arguments; an empty string asks for nothing, which
 * is how pages that do not tune anything stay off the API.
 */
export function useTuning(args: string): Tuning {
  const [comparison, setComparison] = useState<EnvironmentConfig[]>([]);
  const [exported, setExported] = useState<unknown>(null);
  const [apiVersion, setApiVersion] = useState<string | null>(null);
  const [exportArgs, setExportArgs] = useState("");
  const [pending, setPending] = useState(0);
  const [comparisonError, setComparisonError] = useState<string | null>(null);
  const [exportError, setExportError] = useState<string | null>(null);
  const [attempt, setAttempt] = useState(0);

  /* A request whose effect was cleaned up was superseded by a newer form, so
     neither its answer nor its failure may reach the screen. */
  const request = useCallback(
    (
      path: string,
      query: string,
      onResult: (response: TuningResponse) => void,
      onError: (message: string | null) => void,
    ) => {
      const controller = new AbortController();
      setPending((count) => count + 1);
      onError(null);
      getTuning(path, query, { baseUrl, signal: controller.signal })
        .then((response) => {
          if (controller.signal.aborted) return;
          if (response.version) setApiVersion(response.version);
          onResult(response);
        })
        .catch((cause: unknown) => {
          if (controller.signal.aborted) return;
          onError(cause instanceof Error ? cause.message : String(cause));
        })
        .finally(() => setPending((count) => count - 1));
      return () => controller.abort();
    },
    [],
  );

  useEffect(() => {
    if (!args) return;
    return request(
      "get-config-all-environments",
      `show_doc=true&format=json&${args}`,
      ({ output }) => setComparison(Array.isArray(output) ? output : []),
      setComparisonError,
    );
  }, [args, attempt, request]);

  useEffect(() => {
    if (!args || !exportArgs) return;
    return request(
      "get-config",
      `${exportArgs}&${args}`,
      ({ output }) => setExported(output),
      setExportError,
    );
  }, [args, exportArgs, attempt, request]);

  const setExportForm = useCallback(
    (form: ExportForm) => setExportArgs(buildExportArgs(form)),
    [],
  );
  const retry = useCallback(() => setAttempt((count) => count + 1), []);

  return {
    comparison,
    exported,
    apiVersion,
    loading: pending > 0,
    error: comparisonError ?? exportError,
    setExportForm,
    retry,
  };
}
