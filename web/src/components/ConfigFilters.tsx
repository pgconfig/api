import type { ChangeEvent, ReactNode } from "react";
import {
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@momoi-labs/kiso-react";

import type { Option } from "../lib/exportForm.js";
import type { ConfigForm } from "../lib/formQuery.js";
import {
  ARCH_OPTIONS,
  DRIVE_TYPE_OPTIONS,
  OS_OPTIONS,
  PG_VERSION_OPTIONS,
} from "../lib/options.js";
import { useConfigForm } from "../lib/useConfigForm.js";
import { Icon, type IconName } from "./Icon.js";

type NumberField = "cpus" | "total_ram" | "max_connections";

/** One fact about the server: its icon, a short label, and the control. */
function Chip({ icon, label, children }: { icon: IconName; label: string; children: ReactNode }) {
  return (
    <span className="filter-chip">
      <Icon name={icon} size="sm" />
      <span className="filter-chip-label" aria-hidden="true">
        {label}
      </span>
      {children}
    </span>
  );
}

function ChipSelect({
  name,
  value,
  options,
  onChange,
}: {
  /** The full name, for assistive technology. The chip shows a short one. */
  name: string;
  value: string;
  options: Option[];
  onChange: (value: string) => void;
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger aria-label={name}>
        <span className="grow truncate">
          <SelectValue />
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
  );
}

/**
 * The server being tuned, as one row of chips under the header. Every change
 * goes to the address bar.
 */
export function ConfigFilters() {
  const { form, update } = useConfigForm();

  const number = (field: NumberField) => (event: ChangeEvent<HTMLInputElement>) => {
    update({ [field]: parseInt(event.target.value, 10) || 1 } as Partial<ConfigForm>);
  };

  return (
    <div className="filter-bar" role="toolbar" aria-label="Environment filters">
      <Chip icon="server-line" label="OS">
        <ChipSelect
          name="Operating system"
          value={form.os_type}
          options={OS_OPTIONS}
          onChange={(os_type) => update({ os_type })}
        />
      </Chip>
      <Chip icon="cpu-line" label="Arch">
        <ChipSelect
          name="Architecture"
          value={form.arch}
          options={ARCH_OPTIONS}
          onChange={(arch) => update({ arch })}
        />
      </Chip>
      <Chip icon="hard-drive-2-line" label="Storage">
        <ChipSelect
          name="Storage type"
          value={form.drive_type}
          options={DRIVE_TYPE_OPTIONS}
          onChange={(drive_type) => update({ drive_type })}
        />
      </Chip>
      <Chip icon="cpu-line" label="CPUs">
        <Input
          type="number"
          min={1}
          aria-label="Number of CPUs"
          value={form.cpus}
          onChange={number("cpus")}
        />
      </Chip>
      <Chip icon="stack-line" label="RAM">
        <Input
          type="number"
          min={1}
          aria-label="Total memory in gigabytes"
          value={form.total_ram}
          onChange={number("total_ram")}
        />
        <span className="filter-chip-unit">GB</span>
      </Chip>
      <Chip icon="links-line" label="Conn.">
        <Input
          type="number"
          min={1}
          aria-label="Max connections"
          value={form.max_connections}
          onChange={number("max_connections")}
        />
      </Chip>
      <Chip icon="database-2-line" label="PG">
        <ChipSelect
          name="PostgreSQL version"
          value={String(form.pg_version)}
          options={PG_VERSION_OPTIONS}
          onChange={(version) => update({ pg_version: parseFloat(version) })}
        />
      </Chip>
    </div>
  );
}
