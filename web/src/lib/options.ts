import type { Option } from "./exportForm.js";

export const ENVIRONMENT_OPTIONS: Option[] = [
  { value: "WEB", label: "General web applications" },
  { value: "OLTP", label: "ERP or long transaction applications" },
  { value: "DW", label: "DataWare house and BI Applications" },
  { value: "Mixed", label: "DB and APP on the same server" },
  { value: "Desktop", label: "Developer local machine" },
];

/** The comparison columns, in order, and the profile each one stands for. */
export const ENV_COLUMN_TO_PROFILE: Record<string, string> = {
  web: "WEB",
  oltp: "OLTP",
  dw: "DW",
  mixed: "Mixed",
  desktop: "Desktop",
};

/** The profile's name as the API spells it, whatever case the URL used. */
export function profileColumnLabel(value: string): string {
  return (
    ENVIRONMENT_OPTIONS.find(
      (option) => option.value.toUpperCase() === String(value ?? "").toUpperCase(),
    )?.value ?? value
  );
}

export const OS_OPTIONS: Option[] = [
  { value: "linux", label: "GNU/Linux Based" },
  { value: "windows", label: "Windows Based" },
  { value: "unix", label: "Unix Based" },
];

export const ARCH_OPTIONS: Option[] = [
  { value: "x86-64", label: "64 Bits (x86-64)" },
  { value: "386", label: "32 Bits (386)" },
];

export const DRIVE_TYPE_OPTIONS: Option[] = [
  { value: "HDD", label: "HDD Storage" },
  { value: "SSD", label: "SSD Storage" },
  { value: "SAN", label: "Network Storage - NAS/SAN" },
];

export const PG_VERSION_OPTIONS: Option[] = [
  { value: "19", label: "19 (Beta)" },
  { value: "18", label: "18 (Stable, default)" },
  { value: "17", label: "17" },
  { value: "16", label: "16" },
  { value: "15", label: "15" },
  { value: "14", label: "14" },
  { value: "13", label: "13 (EOL)" },
  { value: "12", label: "12 (EOL)" },
  { value: "11", label: "11 (EOL)" },
  { value: "10", label: "10 (EOL)" },
  { value: "9.6", label: "9.6 (EOL)" },
  { value: "9.5", label: "9.5 (EOL)" },
  { value: "9.4", label: "9.4 (EOL)" },
  { value: "9.3", label: "9.3 (EOL)" },
  { value: "9.2", label: "9.2 (EOL)" },
  { value: "9.1", label: "9.1 (EOL)" },
];
