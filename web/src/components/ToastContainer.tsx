"use client";

import React from "react";
import { Bell, ArrowUpRight, ArrowDownRight, Handshake, X } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";

export const ToastContainer: React.FC = () => {
  const { toasts, dismissToast } = useLedger();

  if (toasts.length === 0) return null;

  return (
    <div className="fixed top-20 right-4 z-50 flex flex-col space-y-2 max-w-sm w-full pointer-events-none no-print">
      {toasts.map((toast) => {
        let borderClass = "border-slate-300";
        let icon = <Bell className="w-5 h-5 text-slate-500 shrink-0" />;

        if (toast.type === "income") {
          borderClass = "border-brand-steel-blue";
          icon = <ArrowUpRight className="w-5 h-5 text-emerald-600 shrink-0" />;
        } else if (toast.type === "expense") {
          borderClass = "border-brand-crimson";
          icon = <ArrowDownRight className="w-5 h-5 text-brand-active-crimson shrink-0" />;
        } else if (toast.type === "handover") {
          borderClass = "border-brand-dark";
          icon = <Handshake className="w-5 h-5 text-brand-dark dark:text-brand-cream shrink-0" />;
        }

        return (
          <div
            key={toast.id}
            className={`pointer-events-auto p-4 bg-white dark:bg-brand-dark shadow-xl border-l-4 ${borderClass} border border-slate-200 dark:border-slate-800 transition-all duration-300 transform translate-y-0`}
          >
            <div className="flex items-start justify-between">
              <div className="flex items-start space-x-3">
                <span className="p-1 bg-slate-100 dark:bg-slate-800">{icon}</span>
                <div>
                  <h4 className="text-sm font-bold text-slate-900 dark:text-white">
                    {toast.title}
                  </h4>
                  <p className="text-xs text-slate-600 dark:text-slate-300 mt-0.5">
                    {toast.message}
                  </p>
                  <span className="text-[10px] text-slate-400 mt-1 block">{toast.timestamp}</span>
                </div>
              </div>
              <button
                type="button"
                onClick={() => dismissToast(toast.id)}
                className="text-slate-400 hover:text-slate-600 dark:hover:text-white p-1 ml-2"
              >
                <X className="w-3.5 h-3.5" />
              </button>
            </div>
          </div>
        );
      })}
    </div>
  );
};
