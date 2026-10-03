import { useState } from "react";
import {
  FormField,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@momoi-labs/kiso-react";

import type { Option } from "../lib/exportForm.js";
import { parseCount, type ConfigForm } from "../lib/formQuery.js";
import {
  ARCH_OPTIONS,
  DRIVE_TYPE_OPTIONS,
  OS_OPTIONS,
  PG_VERSION_OPTIONS,
} from "../lib/options.js";
import { useConfigForm } from "../lib/useConfigForm.js";
import { Icon, type IconName } from "./Icon.js";

type NumberField = "cpus" | "total_ram" | "max_connections";

/** One fact about the server, chosen from a list: icon, short label, value. */
function FilterSelect({
  icon,
  label,
  name,
  value,
  options,
  onChange,
}: {
  icon: IconName;
  label: string;
  /** The full name, for assistive technology. The field shows a short one. */
  name: string;
  value: string;
  options: Option[];
  onChange: (value: string) => void;
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <FormField
        label={label}
        layout="inline"
        controlSize="sm"
        leading={<Icon name={icon} size="sm" />}
      >
        <SelectTrigger aria-label={name}>
          <SelectValue />
        </SelectTrigger>
      </FormField>
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
 * One fact about the server that is a count: icon, short label, number. The
 * field keeps what is being typed, so it can be emptied and retyped; the form
 * only hears about it once the text is a count.
 */
function FilterNumber({
  icon,
  label,
  name,
  suffix,
  value,
  onChange,
}: {
  icon: IconName;
  label: string;
  name: string;
  suffix?: string;
  value: number;
  onChange: (count: number) => void;
}) {
  const [draft, setDraft] = useState<string | null>(null);

  return (
    <FormField
      label={label}
      layout="inline"
      controlSize="sm"
      leading={<Icon name={icon} size="sm" />}
      suffix={suffix}
      type="number"
      min={1}
      aria-label={name}
      value={draft ?? value}
      onChange={(event) => {
        setDraft(event.target.value);
        const count = parseCount(event.target.value);
        if (count !== null) onChange(count);
      }}
      onBlur={() => setDraft(null)}
    />
  );
}

/**
 * The server being tuned, as one row of fields under the header. Every change
 * goes to the address bar.
 */
export function ConfigFilters() {
  const { form, update } = useConfigForm();

  const number = (field: NumberField) => (count: number) => {
    update({ [field]: count } as Partial<ConfigForm>);
  };

  return (
    <div className="filter-bar" role="toolbar" aria-label="Environment filters">
      <FilterSelect
        icon="server-line"
        label="OS"
        name="Operating system"
        value={form.os_type}
        options={OS_OPTIONS}
        onChange={(os_type) => update({ os_type })}
      />
      <FilterSelect
        icon="cpu-line"
        label="Arch"
        name="Architecture"
        value={form.arch}
        options={ARCH_OPTIONS}
        onChange={(arch) => update({ arch })}
      />
      <FilterSelect
        icon="hard-drive-2-line"
        label="Storage"
        name="Storage type"
        value={form.drive_type}
        options={DRIVE_TYPE_OPTIONS}
        onChange={(drive_type) => update({ drive_type })}
      />
      <FilterNumber
        icon="cpu-line"
        label="CPUs"
        name="Number of CPUs"
        value={form.cpus}
        onChange={number("cpus")}
      />
      <FilterNumber
        icon="stack-line"
        label="RAM"
        name="Total memory in gigabytes"
        suffix="GB"
        value={form.total_ram}
        onChange={number("total_ram")}
      />
      <FilterNumber
        icon="links-line"
        label="Conn."
        name="Max connections"
        value={form.max_connections}
        onChange={number("max_connections")}
      />
      <FilterSelect
        icon="database-2-line"
        label="PG"
        name="PostgreSQL version"
        value={String(form.pg_version)}
        options={PG_VERSION_OPTIONS}
        onChange={(version) => update({ pg_version: parseFloat(version) })}
      />
    </div>
  );
}
