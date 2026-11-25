"use client";

import React from "react";
import {
  Calendar,
  MapPin,
  Building2,
  Smartphone,
  Banknote,
  Hammer,
  AlertTriangle,
  Share2,
  Printer,
} from "lucide-react";
import { LocalizedReportData } from "@/lib/api";
import { t } from "@/lib/i18n";

interface ExtendedReportData extends LocalizedReportData {
  family_name?: string;
  total_inflows?: string;
  total_outflows?: string;
  custody_breakdown?: Array<{
    name: string;
    custodian: string;
    amount: string;
    type: string;
  }>;
  projects?: Array<{
    name: string;
    target: string;
    collected: string;
    pct: number;
  }>;
  debts?: Array<{
    title: string;
    amount: string;
    due_date: string;
  }>;
}

interface MothersReportCardProps {
  data: ExtendedReportData;
  locale: "rw" | "en";
}

export const MothersReportCard: React.FC<MothersReportCardProps> = ({ data, locale }) => {
  const handlePrint = () => {
    window.print();
  };

  const handleWhatsAppShare = () => {
    const text = `*${data.title}*\n${t(locale, "transactions.th_date")}: ${data.generated_date}\n${t(locale, "onboarding.family_name")}: ${data.family_name || t(locale, "scopes.family_ledger")}\n\n*${data.total_assets_label}:* ${data.total_assets_value} RWF\nMoMo: ${data.momo_value}\n${t(locale, "wallets.channel_cash")}: ${data.cash_value}\n${t(locale, "wallets.channel_bank")}: ${data.bank_value}\n${data.liabilities_label}: ${data.liabilities_value} RWF\n\n${t(locale, "report.view_full_summary", { url: window.location.href })}`;

    window.open(`https://wa.me/?text=${encodeURIComponent(text)}`, "_blank");
  };

  return (
    <div className="bg-white dark:bg-brand-dark shadow-2xl border border-slate-200 dark:border-slate-800 overflow-hidden max-w-2xl mx-auto my-6 print-card transition-colors">
      {/* Elder-Friendly Brand Header */}
      <div className="bg-gradient-to-r from-brand-dark to-[#004e75] p-8 text-white text-center flex flex-col items-center relative">
        <div className="w-16 h-16 bg-white/10 p-2 shadow-inner border border-white/20 mb-3 flex items-center justify-center">
          <img
            src="/logo-white.png"
            alt="Umuryango Ledger Logo"
            width={48}
            height={48}
            className="object-contain"
          />
        </div>
        <div className="inline-block bg-brand-cream/20 border border-brand-cream/30 text-brand-cream px-3 py-1 text-xs font-black tracking-widest uppercase mb-2">
          {data.family_name || t(locale, "scopes.family_ledger")}
        </div>
        <h2 className="text-2xl sm:text-3xl font-black tracking-tight text-brand-cream">
          {data.mothers_headline}
        </h2>
        <p className="mt-2 text-slate-200 text-xs sm:text-sm max-w-md font-medium leading-relaxed">
          {data.mothers_description}
        </p>

        <div className="mt-4 flex items-center space-x-2 text-xs text-brand-cream/80 bg-white/10 px-3 py-1">
          <Calendar className="w-3.5 h-3.5" />
          <span>{data.generated_date}</span>
        </div>
      </div>

      <div className="p-6 sm:p-8 space-y-6">
        {/* Total Assets Spotlight */}
        <div className="p-6 bg-gradient-to-b from-[#fdf0d5]/40 to-[#fdf0d5]/10 dark:from-slate-800 dark:to-slate-900 border-2 border-brand-cream/70 dark:border-slate-700 text-center">
          <span className="text-xs font-extrabold uppercase tracking-widest text-brand-dark dark:text-brand-cream">
            {data.total_assets_label}
          </span>
          <div className="text-4xl sm:text-5xl font-black text-brand-dark dark:white mt-2 tracking-tight">
            {data.total_assets_value}{" "}
            <span className="text-xl sm:text-2xl font-bold text-slate-500">RWF</span>
          </div>
          <div className="flex justify-center items-center space-x-6 mt-4 text-xs font-semibold text-slate-600 dark:text-slate-300">
            <div>
              <span className="text-emerald-700 dark:text-emerald-400 font-bold">↑ +450,000</span>{" "}
              {t(locale, "report.inflow")}
            </div>
            <span>•</span>
            <div>
              <span className="text-brand-crimson dark:text-rose-400 font-bold">↓ -45,000</span>{" "}
              {t(locale, "report.outflow")}
            </div>
          </div>
        </div>

        {/* Where is the cash? (Custody breakdown) */}
        <div>
          <h4 className="text-xs font-extrabold uppercase tracking-wider text-slate-500 dark:text-slate-400 mb-3 flex items-center space-x-1.5">
            <MapPin className="w-3.5 h-3.5 text-brand-steel-blue" />
            <span>{t(locale, "report.where_funds_held")}</span>
          </h4>

          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <div className="p-4 bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 text-center">
              <Building2 className="w-6 h-6 mx-auto mb-1 text-slate-600 dark:text-slate-300" />
              <span className="text-xs text-slate-500 dark:text-slate-400 font-medium block">
                {data.bank_label}
              </span>
              <span className="text-base font-extrabold text-brand-dark dark:text-white mt-1 block">
                {data.bank_value}
              </span>
              <span className="text-[10px] text-slate-400 mt-0.5 block">
                {t(locale, "wallets.custodian_label", { name: "Papa" })}
              </span>
            </div>

            <div className="p-4 bg-amber-50/70 dark:bg-slate-800 border border-amber-200 dark:border-slate-700 text-center">
              <Smartphone className="w-6 h-6 mx-auto mb-1 text-amber-700 dark:text-amber-400" />
              <span className="text-xs text-amber-900 dark:text-amber-300 font-medium block">
                {data.momo_label}
              </span>
              <span className="text-base font-extrabold text-amber-900 dark:text-white mt-1 block">
                {data.momo_value}
              </span>
              <span className="text-[10px] text-amber-800/80 dark:text-slate-400 mt-0.5 block">
                {t(locale, "wallets.custodian_label", { name: "Mama" })}
              </span>
            </div>

            <div className="p-4 bg-rose-50/70 dark:bg-slate-800 border border-rose-200 dark:border-slate-700 text-center">
              <Banknote className="w-6 h-6 mx-auto mb-1 text-brand-crimson dark:text-rose-400" />
              <span className="text-xs text-rose-900 dark:text-rose-300 font-medium block">
                {data.cash_label}
              </span>
              <span className="text-base font-extrabold text-brand-crimson dark:text-white mt-1 block">
                {data.cash_value}
              </span>
              <span className="text-[10px] text-rose-800/80 dark:text-slate-400 mt-0.5 block">
                {t(locale, "wallets.in_hand_label", { custodians: "Mama & Papa" })}
              </span>
            </div>
          </div>
        </div>

        {/* Active Projects Checklist */}
        <div className="p-5 bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700">
          <h4 className="text-xs font-extrabold uppercase tracking-wider text-slate-600 dark:text-slate-300 mb-3 flex items-center space-x-1.5">
            <Hammer className="w-3.5 h-3.5 text-brand-steel-blue" />
            <span>{t(locale, "report.active_projects_section")}</span>
          </h4>
          <div className="space-y-3">
            <div>
              <div className="flex justify-between text-xs font-bold text-slate-800 dark:text-slate-200">
                <span>{t(locale, "report.house_renovation")}</span>
                <span>1,200,000 / 5,000,000 RWF (24%)</span>
              </div>
              <div className="w-full h-2 bg-slate-200 dark:bg-slate-700 mt-1.5 overflow-hidden">
                <div className="h-full bg-brand-steel-blue w-[24%]" />
              </div>
            </div>
          </div>
        </div>

        {/* Liabilities / Outstanding Loans */}
        <div className="p-4 bg-rose-50/80 dark:bg-rose-950/20 border border-rose-200 dark:border-rose-900 flex items-center justify-between">
          <div className="flex items-center space-x-2.5">
            <AlertTriangle className="w-5 h-5 text-rose-600" />
            <div>
              <span className="text-xs font-extrabold uppercase tracking-wider text-rose-900 dark:text-rose-300 block">
                {data.liabilities_label}
              </span>
              <span className="text-[11px] text-rose-700 dark:text-rose-400">
                {t(locale, "report.fertilizer_loan")}
              </span>
            </div>
          </div>
          <span className="text-lg font-black text-rose-900 dark:text-rose-300">
            {data.liabilities_value} RWF
          </span>
        </div>

        {/* Action Buttons: WhatsApp & Print PDF */}
        <div className="pt-2 grid grid-cols-1 sm:grid-cols-2 gap-3 no-print">
          <button
            type="button"
            onClick={handleWhatsAppShare}
            className="w-full py-3 bg-[#25D366] hover:bg-[#20ba59] text-white font-extrabold text-xs shadow-md transition flex items-center justify-center space-x-2"
          >
            <Share2 className="w-4 h-4" />
            <span>{t(locale, "report.share_whatsapp")}</span>
          </button>

          <button
            type="button"
            onClick={handlePrint}
            className="w-full py-3 bg-brand-dark dark:bg-slate-700 hover:bg-brand-dark/90 text-brand-cream dark:text-white font-extrabold text-xs shadow-md transition flex items-center justify-center space-x-2"
          >
            <Printer className="w-4 h-4" />
            <span>{t(locale, "report.download_pdf")}</span>
          </button>
        </div>
      </div>
    </div>
  );
};
