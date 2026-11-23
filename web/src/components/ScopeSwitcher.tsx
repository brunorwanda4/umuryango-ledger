"use client";

import React from "react";
import { Landmark, Lock } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";

interface ScopeSwitcherProps {
  locale?: "rw" | "en";
}

export const ScopeSwitcher: React.FC<ScopeSwitcherProps> = ({ locale = "rw" }) => {
  const { scope, setScope } = useLedger();

  const isRw = locale === "rw";
  const personalLabel = isRw ? "Imari Yanjye" : "Personal";
  const familyLabel = isRw ? "Igitabo cy'Umuryango" : "Family Ledger";

  return (
    <div className="inline-flex p-1 bg-slate-100 dark:bg-slate-800 rounded-xl border border-slate-200 dark:border-slate-700 shadow-inner">
      <button
        type="button"
        onClick={() => setScope("family")}
        className={`px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all duration-200 flex items-center space-x-1.5 ${
          scope === "family"
            ? "bg-brand-dark text-brand-cream shadow-md"
            : "text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white"
        }`}
      >
        <Landmark className="w-3.5 h-3.5" />
        <span>{familyLabel}</span>
      </button>

      <button
        type="button"
        onClick={() => setScope("personal")}
        className={`px-3.5 py-1.5 rounded-lg text-xs font-bold transition-all duration-200 flex items-center space-x-1.5 ${
          scope === "personal"
            ? "bg-brand-crimson text-white shadow-md"
            : "text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white"
        }`}
      >
        <Lock className="w-3.5 h-3.5" />
        <span>{personalLabel}</span>
      </button>
    </div>
  );
};
