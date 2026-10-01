import { Link as RouterLink } from "react-router";
import {
  Button,
  EmptyState,
  EmptyStateActions,
  EmptyStateDescription,
  EmptyStateTitle,
} from "@momoi-labs/kiso-react";

/** An address the app has no page for. */
export function NotFound() {
  return (
    <EmptyState variant="informational">
      <EmptyStateTitle asChild>
        <h1>Page not found</h1>
      </EmptyStateTitle>
      <EmptyStateDescription>
        Nothing lives at this address. It may have moved, or the link may be wrong.
      </EmptyStateDescription>
      <EmptyStateActions>
        <Button asChild size="sm">
          <RouterLink to="/">Go to the profile comparison</RouterLink>
        </Button>
      </EmptyStateActions>
    </EmptyState>
  );
}
