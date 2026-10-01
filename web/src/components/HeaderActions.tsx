import { Link as RouterLink, useLocation } from "react-router";
import { Button, Spinner } from "@momoi-labs/kiso-react";

import { DESKTOP, useMediaQuery } from "../lib/useMediaQuery.js";
import { Icon } from "./Icon.js";
import { ProfileSelect } from "./ProfileSelect.js";

/**
 * What the header holds on the pages that tune a server: the profile, and the
 * way to the other page. A desktop has the export beside the comparison, so
 * only a narrow screen needs the way there.
 */
export function HeaderActions({ page, loading }: { page: "compare" | "export"; loading: boolean }) {
  const isDesktop = useMediaQuery(DESKTOP);
  const { search } = useLocation();

  return (
    <>
      {loading && <Spinner label="Loading recommendations" />}
      <ProfileSelect />
      {page === "compare" && !isDesktop && (
        <Button asChild size="sm" className="btn-icon">
          <RouterLink to={{ pathname: "/export", search }} aria-label="Export" title="Export">
            <Icon name="file-down" />
          </RouterLink>
        </Button>
      )}
      {page === "export" && (
        <Button asChild size="sm">
          <RouterLink to={{ pathname: "/", search }}>Compare</RouterLink>
        </Button>
      )}
    </>
  );
}
