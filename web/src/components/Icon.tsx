import type { ReactNode } from "react";

/**
 * The app's own icons. Kiso styles an `.icon` and leaves the drawings to the
 * product; these are the few this app needs, on Kiso's 16px grid.
 */
const paths = {
  "chevron-right": <path d="M6 3.5 10.5 8 6 12.5" />,
  "chevron-down": <path d="M3.5 6 8 10.5 12.5 6" />,
  x: <path d="M4 4l8 8M12 4l-8 8" />,
  copy: (
    <>
      <rect x="5.5" y="5.5" width="8" height="8" rx="1.5" />
      <path d="M3.5 10.5H3a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h6.5a1 1 0 0 1 1 1v.5" />
    </>
  ),
  export: (
    <>
      <path d="M9 2H4.5A1.5 1.5 0 0 0 3 3.5v9A1.5 1.5 0 0 0 4.5 14h7a1.5 1.5 0 0 0 1.5-1.5V6z" />
      <path d="M9 2v4h4M8 8v4M6.25 10.25 8 12l1.75-1.75" />
    </>
  ),
  doc: (
    <>
      <path d="M9 2H4.5A1.5 1.5 0 0 0 3 3.5v9A1.5 1.5 0 0 0 4.5 14h7a1.5 1.5 0 0 0 1.5-1.5V6z" />
      <path d="M9 2v4h4M5.5 9h5M5.5 11.5h5" />
    </>
  ),
  sliders: (
    <>
      <path d="M2.5 4h6M12 4h1.5M2.5 8h1.5M7.5 8h6M2.5 12h5M11 12h2.5" />
      <circle cx="10.25" cy="4" r="1.75" />
      <circle cx="5.75" cy="8" r="1.75" />
      <circle cx="9.25" cy="12" r="1.75" />
    </>
  ),
  book: (
    <>
      <path d="M8 4.5A1.5 1.5 0 0 0 6.5 3h-4v9.5h4.25A1.25 1.25 0 0 1 8 13.75z" />
      <path d="M8 4.5A1.5 1.5 0 0 1 9.5 3h4v9.5H9.25A1.25 1.25 0 0 0 8 13.75z" />
    </>
  ),
  info: (
    <>
      <circle cx="8" cy="8" r="6.25" />
      <path d="M8 7.5V11M8 5h.01" />
    </>
  ),
  lightbulb: (
    <>
      <path d="M5.2 9.6a4 4 0 1 1 5.6 0c-.5.5-.8 1-.8 1.9H6c0-.9-.3-1.4-.8-1.9z" />
      <path d="M6.5 13.5h3" />
    </>
  ),
  important: (
    <>
      <path d="M2.5 3h11v8.5H7.5l-3 2.5v-2.5h-2z" />
      <path d="M8 5.25v2.5M8 9.5h.01" />
    </>
  ),
  warning: (
    <>
      <path d="M8 2.5 14 13H2z" />
      <path d="M8 6.5v3M8 11.25h.01" />
    </>
  ),
  caution: (
    <>
      <path d="M5.4 1.75h5.2l3.65 3.65v5.2l-3.65 3.65H5.4L1.75 10.6V5.4z" />
      <path d="M8 5v3.5M8 11h.01" />
    </>
  ),
} satisfies Record<string, ReactNode>;

export type IconName = keyof typeof paths;

export function Icon({ name, className }: { name: IconName; className?: string }) {
  return (
    <svg
      className={className ? `icon ${className}` : "icon"}
      viewBox="0 0 16 16"
      aria-hidden="true"
    >
      {paths[name]}
    </svg>
  );
}

/** The GitHub mark is a filled shape, unlike the stroked icons above. */
export function GitHubIcon() {
  return (
    <svg className="icon icon-filled" viewBox="0 0 16 16" aria-hidden="true">
      <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z" />
    </svg>
  );
}
