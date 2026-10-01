import { useEffect, useState } from "react";
import { Route, Routes, useLocation } from "react-router";

import { HeaderActions } from "./components/HeaderActions.js";
import { Shell } from "./components/Shell.js";
import { findGuidePage } from "./guide/pages.js";
import { buildUrlArgs } from "./lib/formQuery.js";
import { crumbsFor, isTuningPath } from "./lib/routes.js";
import { useConfigForm } from "./lib/useConfigForm.js";
import { useTuning } from "./lib/useTuning.js";
import { Compare } from "./views/Compare.js";
import { Export } from "./views/Export.js";
import { Guide } from "./views/Guide.js";
import { NotFound } from "./views/NotFound.js";

export function App() {
  const { pathname, search } = useLocation();
  const { form } = useConfigForm();
  const tunes = isTuningPath(pathname);
  const tuning = useTuning(tunes ? buildUrlArgs(form) : "");

  // The form's query string outlives a visit to the guide, so the way back to
  // the comparison returns to the same configuration.
  const [tuningSearch, setTuningSearch] = useState(tunes ? search : "");
  useEffect(() => {
    if (tunes) setTuningSearch(search);
  }, [tunes, search]);

  return (
    <Shell
      apiVersion={tuning.apiVersion}
      crumbs={crumbsFor(pathname, (slug) => findGuidePage(slug)?.title)}
      tuningSearch={tuningSearch}
      actions={
        tunes && (
          <HeaderActions
            page={pathname.startsWith("/export") ? "export" : "compare"}
            loading={tuning.loading}
          />
        )
      }
    >
      <Routes>
        <Route path="/" element={<Compare form={form} tuning={tuning} />} />
        <Route path="/tuning" element={<Compare form={form} tuning={tuning} />} />
        <Route path="/export" element={<Export form={form} tuning={tuning} />} />
        <Route path="/guide" element={<Guide />} />
        <Route path="/guide/:slug" element={<Guide />} />
        <Route
          path="*"
          element={
            <div className="page" id="content">
              <NotFound />
            </div>
          }
        />
      </Routes>
    </Shell>
  );
}
