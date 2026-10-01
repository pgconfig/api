interface ImportMetaEnv {
  /** Where the tuning API answers. Defaults to /v1/tuning/ on this origin. */
  readonly VITE_API_BASE_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
