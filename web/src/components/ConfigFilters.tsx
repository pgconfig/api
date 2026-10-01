import type { ChangeEvent } from "react";
import { FormField } from "@momoi-labs/kiso-react";

import type { ConfigForm } from "../lib/formQuery.js";
import {
  ARCH_OPTIONS,
  DRIVE_TYPE_OPTIONS,
  ENVIRONMENT_OPTIONS,
  OS_OPTIONS,
  PG_VERSION_OPTIONS,
} from "../lib/options.js";
import { useConfigForm } from "../lib/useConfigForm.js";
import { SelectField } from "./SelectField.js";

type NumberField = "cpus" | "total_ram" | "max_connections";

/** The server being tuned. Every change goes to the address bar. */
export function ConfigFilters() {
  const { form, update, selectProfile } = useConfigForm();

  const number = (field: NumberField) => (event: ChangeEvent<HTMLInputElement>) => {
    update({ [field]: parseInt(event.target.value, 10) || 1 } as Partial<ConfigForm>);
  };

  return (
    <div className="config-filters" role="group" aria-label="Environment filters">
      <SelectField
        className="config-filters-profile"
        label="Application profile"
        placeholder="Select profile"
        value={form.environment_name}
        options={ENVIRONMENT_OPTIONS}
        onChange={selectProfile}
      />
      <SelectField
        label="Operating system"
        value={form.os_type}
        options={OS_OPTIONS}
        onChange={(os_type) => update({ os_type })}
      />
      <SelectField
        label="Architecture"
        value={form.arch}
        options={ARCH_OPTIONS}
        onChange={(arch) => update({ arch })}
      />
      <SelectField
        label="Storage"
        value={form.drive_type}
        options={DRIVE_TYPE_OPTIONS}
        onChange={(drive_type) => update({ drive_type })}
      />
      <FormField
        label="CPUs"
        type="number"
        min={1}
        value={form.cpus}
        onChange={number("cpus")}
      />
      <FormField
        label="Memory (GB)"
        type="number"
        min={1}
        value={form.total_ram}
        onChange={number("total_ram")}
      />
      <FormField
        label="Max connections"
        type="number"
        min={1}
        value={form.max_connections}
        onChange={number("max_connections")}
      />
      <SelectField
        label="PostgreSQL version"
        value={String(form.pg_version)}
        options={PG_VERSION_OPTIONS}
        onChange={(version) => update({ pg_version: parseFloat(version) })}
      />
    </div>
  );
}
