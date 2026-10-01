import { useCallback, useState } from "react";

/* The key the previous app used, so a returning visitor keeps their choice.
   It holds "light" or "dark"; no entry means following the system. */
const themeKey = "theme";

/**
 * Storage can be unavailable (private mode, disabled cookies). A failure has
 * to degrade to "system", not throw and leave the page half-built.
 */
export function currentTheme(): string {
  let stored: string | null = null;
  try {
    stored = localStorage.getItem(themeKey);
  } catch {
    stored = null;
  }
  return stored === "light" || stored === "dark" ? stored : "system";
}

/**
 * System means no data-theme attribute at all: the tokens are built on CSS
 * light-dark(), so `color-scheme` on :root already follows the OS. Only an
 * explicit choice narrows it.
 */
export function applyTheme(theme: string): void {
  if (theme === "system") delete document.documentElement.dataset.theme;
  else document.documentElement.dataset.theme = theme;
}

export function useTheme(): [string, (theme: string) => void] {
  const [theme, setTheme] = useState(currentTheme);

  const choose = useCallback((next: string) => {
    try {
      if (next === "system") localStorage.removeItem(themeKey);
      else localStorage.setItem(themeKey, next);
    } catch {
      // A theme that cannot be remembered still applies to this page.
    }
    applyTheme(next);
    setTheme(next);
  }, []);

  return [theme, choose];
}
