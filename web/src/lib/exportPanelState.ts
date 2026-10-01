export const EXPORT_PANEL_STATE_COOKIE = "export_panel_state";
/* The previous app stored a pixel width as `export_panel_width`. The split
   stores the share of the comparison pane, so it needs a name of its own. */
export const EXPORT_PANEL_SIZE_COOKIE = "export_panel_size";
export const EXPORT_PANEL_COOKIE_MAX_AGE = 60 * 60 * 24 * 7;

/** The comparison pane's share of the split, in percent. */
export const EXPORT_PANEL_DEFAULT_SIZE = 70;
export const EXPORT_PANEL_MIN_SIZE = 45;
export const EXPORT_PANEL_MAX_SIZE = 78;

export function readPanelOpen(cookie: string): boolean {
  return !cookie.includes(`${EXPORT_PANEL_STATE_COOKIE}=false`);
}

export function readPanelSize(cookie: string): number {
  const match = cookie.match(new RegExp(`${EXPORT_PANEL_SIZE_COOKIE}=(\\d+)`));
  if (!match) return EXPORT_PANEL_DEFAULT_SIZE;
  const size = Number.parseInt(match[1], 10);
  return Math.min(Math.max(size, EXPORT_PANEL_MIN_SIZE), EXPORT_PANEL_MAX_SIZE);
}

export function panelCookie(name: string, value: boolean | number): string {
  return `${name}=${value}; path=/; max-age=${EXPORT_PANEL_COOKIE_MAX_AGE}`;
}
