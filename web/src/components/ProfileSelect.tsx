import { useId } from "react";
import {
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@momoi-labs/kiso-react";

import { ENVIRONMENT_OPTIONS } from "../lib/options.js";
import { useConfigForm } from "../lib/useConfigForm.js";
import { Icon } from "./Icon.js";

/** The application profile, in the header: it picks the column the page is about. */
export function ProfileSelect() {
  const { form, selectProfile } = useConfigForm();
  const id = useId();

  return (
    <div className="profile-select">
      <Icon name="window-line" size="sm" />
      <Label htmlFor={id} className="profile-select-label">
        Application profile
      </Label>
      <Select value={form.environment_name} onValueChange={selectProfile}>
        <SelectTrigger id={id} aria-label="Application profile">
          <span className="grow truncate">
            <SelectValue placeholder="Select profile" />
          </span>
        </SelectTrigger>
        <SelectContent>
          {ENVIRONMENT_OPTIONS.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
