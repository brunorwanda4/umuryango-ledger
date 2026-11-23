import React from "react";
import Link from "next/link";
import { ShieldCheck, Globe } from "lucide-react";
import { MothersReportCard } from "@/components/MothersReportCard";
import { LocalizedReportData } from "@/lib/api";

export default async function ZeroLoginReportPage({
  params,
}: {
  params: Promise<{ locale: string; token: string }>;
}) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const token = resolvedParams.token;
  const isRw = locale === "rw";

  const otherLocale = isRw ? "en" : "rw";

  const reportData: LocalizedReportData & { family_name: string } = {
    title: isRw ? "Raporo y'Imari y'Umuryango" : "Family Financial Summary",
    generated_date: isRw ? "23 Ugushyingo 2026" : "November 23, 2026",
    family_name: isRw ? "Umuryango Mugisha & Keza" : "Mugisha & Keza Family",
    total_assets_label: isRw ? "Umutungo Wose w'Umuryango" : "Total Family Balance",
    total_assets_value: "10,000,000",
    cash_label: isRw ? "Mu Ntoki (Cash)" : "Cash in Hand",
    cash_value: "350,000 RWF",
    momo_label: isRw ? "MTN Mobile Money" : "MTN Mobile Money",
    momo_value: "1,450,000 RWF",
    bank_label: isRw ? "Konti muri BK" : "Bank of Kigali",
    bank_value: "8,200,000 RWF",
    liabilities_label: isRw ? "Amadeni Umuryango Ufite" : "Outstanding Liabilities",
    liabilities_value: "150,000",
    mothers_headline: isRw ? "Raporo y'Ababyeyi n'Abakambwe" : "Mother's Summary Report",
    mothers_description: isRw
      ? "Incamake y'umutungo n'imari y'umuryango yagenewe ababyeyi kugira ngo bamenye aho buri faranga riherereye n'uko ibintu byifashe."
      : "A transparent, high-trust financial view crafted for family elders to easily track balances, cash custody, and ongoing projects without passwords.",
  };

  return (
    <div className="py-6 space-y-6 pt-safe pb-safe pl-safe pr-safe">
      {/* Top Banner with Token info & Lang toggle */}
      <div className="flex flex-col sm:flex-row items-center justify-between gap-3 max-w-2xl mx-auto px-4 no-print">
        <div className="flex items-center space-x-2">
          <span className="inline-flex items-center px-3 py-1 bg-brand-cream/80 dark:bg-slate-800 text-brand-dark dark:text-brand-cream border border-brand-cream dark:border-slate-700 rounded-full text-xs font-bold">
            <ShieldCheck className="w-3.5 h-3.5 mr-1 text-emerald-600" />
            {isRw ? "Ihuza ry'Umutekano ryizewe (Zero-Login)" : "Secure Zero-Login Link"}
          </span>
          <span className="text-xs text-slate-400">Token: {token}</span>
        </div>

        <Link
          href={`/${otherLocale}/report/${token}`}
          className="inline-flex items-center text-xs font-bold px-3 py-1.5 rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:border-brand-active-crimson transition shadow-sm"
        >
          <Globe className="w-3.5 h-3.5 mr-1 text-slate-500" />
          {isRw ? "Switch to English" : "Hindura mu Kinyarwanda"}
        </Link>
      </div>

      <MothersReportCard data={reportData} locale={locale} />
    </div>
  );
}
