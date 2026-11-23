const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080/api/v1";

export interface LocalizedReportData {
  title: string;
  generated_date: string;
  total_assets_label: string;
  total_assets_value: string;
  cash_label: string;
  cash_value: string;
  momo_label: string;
  momo_value: string;
  bank_label: string;
  bank_value: string;
  liabilities_label: string;
  liabilities_value: string;
  mothers_headline: string;
  mothers_description: string;
}

export async function fetchZeroLoginReport(
  token: string,
  locale: "rw" | "en" = "rw",
): Promise<LocalizedReportData> {
  const res = await fetch(`${API_BASE}/reports/zero-login/${token}`, {
    headers: {
      "Accept-Language": locale,
    },
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error(`Failed to load report: ${res.statusText}`);
  }

  return res.json();
}
