import { useEffect, useState } from "react";

/** Whether the viewport matches a media query, kept current as it changes. */
export function useMediaQuery(query: string): boolean {
  const [matches, setMatches] = useState(() => window.matchMedia(query).matches);
  useEffect(() => {
    const list = window.matchMedia(query);
    const update = () => setMatches(list.matches);
    update();
    list.addEventListener("change", update);
    return () => list.removeEventListener("change", update);
  }, [query]);
  return matches;
}

/** From this width the sidebar is a rail and the export panel sits beside the table. */
export const DESKTOP = "(min-width: 1024px)";
