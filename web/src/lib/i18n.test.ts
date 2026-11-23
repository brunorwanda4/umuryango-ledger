import { describe, expect, it } from "bun:test";
import { t } from "./i18n";

describe("Web i18n JSON Integration", () => {
  it("translates common keys in English and Kinyarwanda", () => {
    expect(t("en", "common.app_name")).toBe("Umuryango Ledger");
    expect(t("rw", "common.app_name")).toBe("Umuryango Ledger");

    expect(t("en", "common.save")).toBe("Save");
    expect(t("rw", "common.save")).toBe("Bika");

    expect(t("en", "scopes.personal_money")).toBe("My Personal Money");
    expect(t("rw", "scopes.personal_money")).toBe("Amafaranga Yanjye");
  });

  it("interpolates parameters properly", () => {
    const enNotif = t("en", "notifications.income_added", {
      actor: "Papa",
      amount: "50,000",
      wallet: "Bank",
      source: "Salary",
    });
    expect(enNotif).toBe("Papa added 50,000 RWF to Bank (Salary).");

    const rwNotif = t("rw", "notifications.income_added", {
      actor: "Papa",
      amount: "50,000",
      wallet: "Bank",
      source: "Salary",
    });
    expect(rwNotif).toBe("Papa yongeye 50,000 RWF kuri konti 'Bank' (Salary).");
  });

  it("falls back to English when a key is missing", () => {
    expect(t("rw", "nonexistent.key")).toBe("nonexistent.key");
  });
});
