import enTranslations from "../../../locales/en.json";
import rwTranslations from "../../../locales/rw.json";

export type Locale = "rw" | "en";

export const translations: Record<Locale, typeof enTranslations> = {
  en: enTranslations,
  rw: rwTranslations,
};

export function getNestedTranslation(obj: unknown, path: string): string | undefined {
  if (!obj || typeof obj !== "object") return undefined;
  const parts = path.split(".");
  let current: unknown = obj;

  for (const part of parts) {
    if (current && typeof current === "object" && part in (current as Record<string, unknown>)) {
      current = (current as Record<string, unknown>)[part];
    } else {
      return undefined;
    }
  }

  return typeof current === "string" ? current : undefined;
}

export function interpolate(template: string, params?: Record<string, string | number>): string {
  if (!params) return template;
  let result = template;
  for (const [key, val] of Object.entries(params)) {
    const standardPlaceholder = `{${key}}`;
    const dollarPlaceholder = `{\\$${key}}`;
    result = result.replaceAll(standardPlaceholder, String(val));
    result = result.replaceAll(dollarPlaceholder, String(val));
  }
  return result;
}

/**
 * Global translation function with fallback to English then key
 */
export function t(locale: Locale, key: string, params?: Record<string, string | number>): string {
  const primaryRoot = translations[locale] ?? translations.rw;
  let text = getNestedTranslation(primaryRoot, key);

  if (text === undefined && locale !== "en") {
    text = getNestedTranslation(translations.en, key);
  }

  if (text === undefined) {
    return key;
  }

  return interpolate(text, params);
}

/**
 * React hook for consuming translations within client components
 */
export function useTranslation(locale: Locale = "rw") {
  return {
    locale,
    t: (key: string, params?: Record<string, string | number>) => t(locale, key, params),
  };
}
