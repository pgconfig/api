import type { ParameterDoc } from "./parameterDoc.js";

/** Match the recommendation's byte or scalar format, preserving negative sentinels. */
export function parameterDefault(doc: ParameterDoc | null, format?: string): string {
  const value = doc?.fields.default;
  if (typeof value !== "string") return "";
  if (value === "") return '""';
  if (format !== "Byte") return value;
  const unit = doc?.fields.unit;
  const number = Number(value);
  if (typeof unit !== "string" || !Number.isFinite(number) || number < 0) return value;
  const bytesPerUnit: Record<string, number> = { B: 1, kB: 1024, "8kB": 8192, MB: 1024 ** 2 };
  const multiplier = bytesPerUnit[unit];
  if (multiplier === undefined) return `${value}${unit}`;
  const bytes = number * multiplier;
  for (const [suffix, size] of [
    ["TB", 1024 ** 4],
    ["GB", 1024 ** 3],
    ["MB", 1024 ** 2],
    ["kB", 1024],
  ] as const) {
    if (bytes >= size && bytes % size === 0) return `${bytes / size}${suffix}`;
  }
  return `${bytes}B`;
}
