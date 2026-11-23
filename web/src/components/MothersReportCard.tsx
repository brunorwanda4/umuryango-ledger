import React from "react";
import { LocalizedReportData } from "@/lib/api";

interface MothersReportCardProps {
  data: LocalizedReportData;
  locale: "rw" | "en";
}

export const MothersReportCard: React.FC<MothersReportCardProps> = ({ data, locale }) => {
  return (
    <div className="rounded-2xl bg-white shadow-xl border border-slate-100 overflow-hidden max-w-xl mx-auto my-6">
      <div className="bg-gradient-to-r from-emerald-600 to-teal-700 p-6 text-white text-center">
        <h2 className="text-2xl font-bold tracking-tight">{data.mothers_headline}</h2>
        <p className="mt-1 text-emerald-100 text-sm">{data.mothers_description}</p>
        <div className="mt-4 inline-block bg-white/20 backdrop-blur-md px-4 py-1 rounded-full text-xs font-semibold">
          {data.generated_date}
        </div>
      </div>

      <div className="p-6 space-y-4">
        {/* Total Net Assets */}
        <div className="p-5 bg-emerald-50/70 rounded-xl border border-emerald-100 text-center">
          <span className="text-xs font-semibold uppercase tracking-wider text-emerald-800">
            {data.total_assets_label}
          </span>
          <div className="text-3xl font-extrabold text-emerald-900 mt-1">
            {data.total_assets_value} RWF
          </div>
        </div>

        {/* Breakdown of Custody */}
        <div className="grid grid-cols-3 gap-3">
          <div className="p-3 bg-slate-50 rounded-lg text-center border border-slate-100">
            <span className="text-xs text-slate-500 font-medium block">
              {data.cash_label}
            </span>
            <span className="text-base font-bold text-slate-800 mt-1 block">
              {data.cash_value}
            </span>
          </div>

          <div className="p-3 bg-amber-50 rounded-lg text-center border border-amber-100">
            <span className="text-xs text-amber-700 font-medium block">
              {data.momo_label}
            </span>
            <span className="text-base font-bold text-amber-900 mt-1 block">
              {data.momo_value}
            </span>
          </div>

          <div className="p-3 bg-blue-50 rounded-lg text-center border border-blue-100">
            <span className="text-xs text-blue-700 font-medium block">
              {data.bank_label}
            </span>
            <span className="text-base font-bold text-blue-900 mt-1 block">
              {data.bank_value}
            </span>
          </div>
        </div>

        {/* Outstanding Loans */}
        <div className="p-4 bg-rose-50/70 rounded-xl border border-rose-100 flex items-center justify-between">
          <span className="text-xs font-semibold uppercase tracking-wider text-rose-800">
            {data.liabilities_label}
          </span>
          <span className="text-lg font-bold text-rose-900">
            {data.liabilities_value} RWF
          </span>
        </div>
      </div>
    </div>
  );
};
