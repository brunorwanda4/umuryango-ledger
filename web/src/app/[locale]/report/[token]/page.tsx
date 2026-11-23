import React from "react";
import Link from "next/link";
import { ShieldCheck, Globe } from "lucide-react";
import { MothersReportCard } from "@/components/MothersReportCard";
import { t } from "@/lib/i18n";
import { LocalizedReportData } from "@/lib/api";

export default async function ZeroLoginReportPage({
  params,
}: {
  params: Promise<{ locale: string; token: string }>;
}) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const token = resolvedParams.token;

  const otherLocale = locale === "rw" ? "en" : "rw";

  const reportData: LocalizedReportData & { family_name: string } = {
    title: t(locale, "report.mothers_report_title"),
    generated_date: t(locale, "dashboard.current_period"),
    family_name: t(locale, "report.demo_family_name"),
    total_assets_label: t(locale, "dashboard.total_balance"),
    total_assets_value: "10,000,000",
    cash_label: t(locale, "wallets.channel_cash"),
    cash_value: "350,000 RWF",
    momo_label: "MTN Mobile Money",
    momo_value: "1,450,000 RWF",
    bank_label: t(locale, "wallets.channel_bank"),
    bank_value: "8,200,000 RWF",
    liabilities_label: t(locale, "loans.borrowed_tab"),
    liabilities_value: "150,000",
    mothers_headline: t(locale, "report.demo_mothers_headline"),
    mothers_description: t(locale, "report.demo_mothers_desc"),
  };

  return (
    <div className="py-6 space-y-6 pt-safe pb-safe pl-safe pr-safe">
      {/* Top Banner with Token info & Lang toggle */}
      <div className="flex flex-col sm:flex-row items-center justify-between gap-3 max-w-2xl mx-auto px-4 no-print">
        <div className="flex items-center space-x-2">
          <span className="inline-flex items-center px-3 py-1 bg-brand-cream/80 dark:bg-slate-800 text-brand-dark dark:text-brand-cream border border-brand-cream dark:border-slate-700 rounded-full text-xs font-bold">
            <ShieldCheck className="w-3.5 h-3.5 mr-1 text-emerald-600" />
            {t(locale, "report.zero_login_secure_link")}
          </span>
          <span className="text-xs text-slate-400">Token: {token}</span>
        </div>

        <Link
          href={`/${otherLocale}/report/${token}`}
          className="inline-flex items-center text-xs font-bold px-3 py-1.5 rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-200 hover:border-brand-active-crimson transition shadow-sm"
        >
          <Globe className="w-3.5 h-3.5 mr-1 text-slate-500" />
          {t(locale, "report.switch_to_other_lang")}
        </Link>
      </div>

      <MothersReportCard data={reportData} locale={locale} />
    </div>
  );
}
