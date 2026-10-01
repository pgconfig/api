import { Alert, AlertContent, AlertDescription, AlertTitle, Button } from "@momoi-labs/kiso-react";

import { Icon } from "./Icon.js";

/** A failed API request: what happened, why, and the way to recover. */
export function LoadError({ reason, onRetry }: { reason: string; onRetry: () => void }) {
  return (
    <Alert variant="error">
      <Icon name="circle-alert" />
      <AlertContent>
        <AlertTitle>Could not get data from the API</AlertTitle>
        <AlertDescription>{reason}. Check the values above, then try again.</AlertDescription>
      </AlertContent>
      <Button size="sm" onClick={onRetry}>
        Try again
      </Button>
    </Alert>
  );
}
