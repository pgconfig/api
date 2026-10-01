import { useId } from "react";
import {
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@momoi-labs/kiso-react";

import type { Option } from "../lib/exportForm.js";

/** A labelled Select: one value out of a fixed list of options. */
export function SelectField({
  label,
  value,
  options,
  onChange,
  placeholder,
  disabled,
  className,
}: {
  label: string;
  value: string;
  options: Option[];
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
}) {
  const id = useId();
  return (
    <div className={className ? `field ${className}` : "field"}>
      <Label htmlFor={id}>{label}</Label>
      <Select value={value} onValueChange={onChange} disabled={disabled}>
        <SelectTrigger id={id}>
          <span className="grow truncate">
            <SelectValue placeholder={placeholder} />
          </span>
        </SelectTrigger>
        <SelectContent>
          {options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
