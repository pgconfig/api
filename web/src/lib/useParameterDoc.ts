import { useEffect, useState } from "react";

import { getParameterDoc, parameterDocUrl, type ParameterDoc } from "./parameterDoc.js";

const baseUrl = import.meta.env.VITE_API_BASE_URL as string | undefined;

/**
 * The PostgreSQL manual's entry for a parameter of a version, or `null`
 * while it loads and when the server has none.
 */
export function useParameterDoc(pgVersion: string, name: string): ParameterDoc | null {
  const [doc, setDoc] = useState<ParameterDoc | null>(null);
  useEffect(() => {
    // The entry of the previous version must not linger while this one loads.
    setDoc(null);
    const controller = new AbortController();
    const url = parameterDocUrl(baseUrl, pgVersion, name, window.location.href);
    getParameterDoc(url, controller.signal).then(setDoc, () => setDoc(null));
    return () => controller.abort();
  }, [pgVersion, name]);
  return doc;
}
