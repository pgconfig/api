export type AlertType = "NOTE" | "TIP" | "IMPORTANT" | "WARNING" | "CAUTION";

export type MarkdownBlock =
  | { type: "markdown"; content: string }
  | { type: "alert"; alertType: AlertType; content: string };

const ALERT_BLOCK_REGEX =
  /(\s*>\s*\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*\n((?:\s*>(?!\s*\[!).*\n?)+))/gi;

function normalizeAlertBody(rawBody: string): string {
  return rawBody
    .split("\n")
    .map((line) => line.replace(/^\s*>\s?/, ""))
    .join("\n")
    .trim();
}

/** Splits Markdown into prose and GitHub-style alert blocks, in order. */
export function parseMarkdownBlocks(markdown: string | null | undefined): MarkdownBlock[] {
  if (!markdown?.trim()) {
    return [];
  }

  const blocks: MarkdownBlock[] = [];
  let lastIndex = 0;

  for (const match of markdown.matchAll(ALERT_BLOCK_REGEX)) {
    const start = match.index ?? 0;

    if (start > lastIndex) {
      const prose = markdown.slice(lastIndex, start).trim();
      if (prose) {
        blocks.push({ type: "markdown", content: prose });
      }
    }

    const alertType = match[2].toUpperCase() as AlertType;
    blocks.push({
      type: "alert",
      alertType,
      content: normalizeAlertBody(match[3]),
    });

    lastIndex = start + match[0].length;
  }

  if (lastIndex < markdown.length) {
    const prose = markdown.slice(lastIndex).trim();
    if (prose) {
      blocks.push({ type: "markdown", content: prose });
    }
  }

  return blocks;
}
