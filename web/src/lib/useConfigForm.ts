import { useCallback, useMemo } from "react";
import { useSearchParams } from "react-router";

import {
  buildUrlArgs,
  formToQuery,
  parseFormQuery,
  type ConfigForm,
} from "./formQuery.js";

/**
 * The form lives in the address bar: reading it parses the query string, and
 * every change is a new history entry, so a configuration can be shared as a
 * link and the back button undoes a change.
 */
export function useConfigForm() {
  const [searchParams, setSearchParams] = useSearchParams();
  const form = useMemo(() => parseFormQuery(searchParams), [searchParams]);

  /** Changes fields and writes the whole form to the address bar. */
  const update = useCallback(
    (patch: Partial<ConfigForm>) => {
      const next = { ...form, ...patch };
      if (buildUrlArgs(next) === buildUrlArgs(form)) return;
      setSearchParams(formToQuery(next));
    },
    [form, setSearchParams],
  );

  /** Changes only the profile, leaving the rest of the query string as it is. */
  const selectProfile = useCallback(
    (environmentName: string) => {
      if (searchParams.get("environment_name") === environmentName) return;
      const next = new URLSearchParams(searchParams);
      next.set("environment_name", environmentName);
      setSearchParams(next);
    },
    [searchParams, setSearchParams],
  );

  return { form, update, selectProfile };
}
