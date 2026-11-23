"use client";

import React from "react";
import { useLedger } from "@/lib/LedgerContext";

export const CustodyChart: React.FC<{ locale: "rw" | "en" }> = ({ locale }) => {
  const { kpis, wallets } = useLedger();
  const isRw = locale === "rw";

  const total = kpis.totalAssets || 1;
  const bankPct = Math.round((kpis.bankTotal / total) * 100);
  const momoPct = Math.round((kpis.momoTotal / total) * 100);
  const cashPct = Math.max(0, 100 - bankPct - momoPct);

  return (
    <div className="p-6 bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm transition-colors">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="font-bold text-slate-900 dark:text-white text-base">
            {isRw ? "Ikwirakwizwa ry'Umutungo" : "Custody Allocation"}
          </h3>
          <p className="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
            {isRw
              ? "Aho amafaranga ari muri iki gihe"
              : "Current distribution across custody types"}
          </p>
        </div>
        <span className="text-xs font-black px-2.5 py-1 rounded-lg bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-brand-cream">
          {kpis.totalAssets.toLocaleString()} RWF
        </span>
      </div>

      {/* Visual multi-segment bar */}
      <div className="h-4 w-full rounded-full bg-slate-100 dark:bg-slate-800 overflow-hidden flex shadow-inner mb-6">
        <div
          style={{ width: `${bankPct}%` }}
          className="bg-brand-dark dark:bg-brand-steel-blue transition-all duration-500"
          title={`Bank: ${bankPct}%`}
        />
        <div
          style={{ width: `${momoPct}%` }}
          className="bg-[#e09f3e] transition-all duration-500"
          title={`MoMo: ${momoPct}%`}
        />
        <div
          style={{ width: `${cashPct}%` }}
          className="bg-brand-active-crimson transition-all duration-500"
          title={`Cash: ${cashPct}%`}
        />
      </div>

      {/* Breakdown items */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
        <div className="p-3 rounded-xl bg-slate-50 dark:bg-slate-800/60 border border-slate-100 dark:border-slate-700/60">
          <div className="flex items-center space-x-2">
            <span className="w-3 h-3 rounded-full bg-brand-dark dark:bg-brand-steel-blue inline-block" />
            <span className="text-xs font-semibold text-slate-600 dark:text-slate-300">
              {isRw ? "Banki" : "Bank"} ({bankPct}%)
            </span>
          </div>
          <div className="text-base font-extrabold text-slate-900 dark:text-white mt-1">
            {kpis.bankTotal.toLocaleString()} RWF
          </div>
        </div>

        <div className="p-3 rounded-xl bg-amber-50/50 dark:bg-slate-800/60 border border-amber-100 dark:border-slate-700/60">
          <div className="flex items-center space-x-2">
            <span className="w-3 h-3 rounded-full bg-[#e09f3e] inline-block" />
            <span className="text-xs font-semibold text-amber-900 dark:text-amber-300">
              MoMo ({momoPct}%)
            </span>
          </div>
          <div className="text-base font-extrabold text-slate-900 dark:text-white mt-1">
            {kpis.momoTotal.toLocaleString()} RWF
          </div>
        </div>

        <div className="p-3 rounded-xl bg-rose-50/50 dark:bg-slate-800/60 border border-rose-100 dark:border-slate-700/60">
          <div className="flex items-center space-x-2">
            <span className="w-3 h-3 rounded-full bg-brand-active-crimson inline-block" />
            <span className="text-xs font-semibold text-rose-900 dark:text-rose-300">
              {isRw ? "Mu Ntoki (Cash)" : "Cash in Hand"} ({cashPct}%)
            </span>
          </div>
          <div className="text-base font-extrabold text-slate-900 dark:text-white mt-1">
            {kpis.cashInHand.toLocaleString()} RWF
          </div>
        </div>
      </div>
    </div>
  );
};
