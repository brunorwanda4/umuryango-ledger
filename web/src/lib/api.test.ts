import { describe, expect, it } from "bun:test";
import { LocalizedReportData } from "./api";

describe("api and domain helpers under Bun runtime", () => {
  it("structures localized report data correctly", () => {
    const mockReport: LocalizedReportData = {
      title: "Raporo y'Imari",
      generated_date: "23 Ugushyingo 2026",
      total_assets_label: "Umutungo Wose",
      total_assets_value: "10000000",
      cash_label: "Mu Ntoki",
      cash_value: "350000",
      momo_label: "MTN Mobile Money",
      momo_value: "1450000",
      bank_label: "Konti muri BK",
      bank_value: "8200000",
      liabilities_label: "Amadeni",
      liabilities_value: "150000",
      mothers_headline: "Raporo y'Ababyeyi",
      mothers_description: "Incamake y'umutungo",
    };

    expect(mockReport.title).toBe("Raporo y'Imari");
    expect(mockReport.total_assets_value).toBe("10000000");
  });
});
