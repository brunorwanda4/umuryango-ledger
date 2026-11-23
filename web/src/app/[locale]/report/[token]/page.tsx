import React from "react";
import { MothersReportCard } from "@/components/MothersReportCard";
import { LocalizedReportData } from "@/lib/api";

export default async function ZeroLoginReportPage({
  params,
}: {
  params: Promise<{ locale: string; token: string }>;
}) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  // Provide initial report data representation (in production, fetched via fetchZeroLoginReport)
  const reportData: LocalizedReportData = {
    title: isRw ? "Raporo y'Imari y'Umuryango" : "Family Financial Summary",
    generated_date: "23 Ugushyingo 2026",
    total_assets_label: isRw ? "Umutungo Wose w'Umuryango" : "Total Family Assets",
    total_assets_value: "10,000,000",
    cash_label: isRw ? "Mu Ntoki (Cash)" : "Cash in Hand",
    cash_value: "350,000 RWF",
    momo_label: isRw ? "Mobile Money" : "Mobile Money",
    momo_value: "1,450,000 RWF",
    bank_label: isRw ? "Muri Banki" : "Bank Deposits",
    bank_value: "8,200,000 RWF",
    liabilities_label: isRw ? "Amadeni Umuryango Ufite" : "Outstanding Liabilities",
    liabilities_value: "150,000",
    mothers_headline: isRw ? "Raporo y'Ababyeyi n'Abakambwe" : "Mother's Summary Report",
    mothers_description: isRw
      ? "Incamake yoroheye buri wese kureba uko umutungo w'umuryango uhagaze."
      : "High-clarity, high-trust financial view designed for family elders.",
  };

  return (
    <div className="py-6 space-y-6">
      <div className="text-center max-w-lg mx-auto">
        <span className="inline-block px-3 py-1 bg-emerald-100 text-emerald-800 rounded-full text-xs font-bold mb-2">
          🔒 {isRw ? "Ihuza ry'Umutekano ryizewe (Zero-Login)" : "Secure Zero-Login Token Access"}
        </span>
        <h1 className="text-2xl font-black text-slate-900">{reportData.title}</h1>
        <p className="text-xs text-slate-500 mt-1">
          {isRw
            ? `Iyi raporo irebwa gusa n'abafite iyi mbumbanyandiko y'umuryango (Token: ${resolvedParams.token})`
            : `Shared securely with family token: ${resolvedParams.token}`}
        </p>
      </div>

      <MothersReportCard data={reportData} locale={locale} />
    </div>
  );
}
