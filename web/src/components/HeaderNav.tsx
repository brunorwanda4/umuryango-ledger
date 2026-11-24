"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Sun, Moon, Globe, ArrowRight } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";

export const HeaderNav: React.FC<{ locale: "rw" | "en" }> = ({ locale }) => {
  const pathname = usePathname();
  const { theme, toggleTheme } = useLedger();

  const otherLocale = locale === "rw" ? "en" : "rw";
  const otherLocaleLabel = locale === "rw" ? "EN" : "RW";

  // Preserve subpath when switching locale
  const targetPath = pathname
    ? pathname.replace(new RegExp(`^/${locale}`), `/${otherLocale}`)
    : `/${otherLocale}`;

  const openAppLabel = locale === "rw" ? "Injira muri App" : "Launch App";

  return (
    <header className="sticky top-0 z-40 bg-white/95 dark:bg-[#003049]/95 backdrop-blur border-b border-slate-200 dark:border-slate-800 transition-colors pt-safe pl-safe pr-safe no-print">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div className="flex items-center space-x-6">
          <Link href={`/${locale}`} className="flex items-center space-x-3 group">
            <img
              src="/icon-192.png"
              alt="Umuryango Ledger Emblem"
              width={36}
              height={36}
              className="shadow-sm border border-slate-200 dark:border-slate-700 group-hover:scale-105 transition-transform"
            />
            <div className="flex flex-col">
              <span className="text-lg font-black tracking-tight text-[#003049] dark:text-[#fdf0d5] leading-none">
                Umuryango <span className="text-[#c1121f]">Ledger</span>
              </span>
              <span className="text-[10px] font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-widest mt-0.5">
                Bilingual Accountability
              </span>
            </div>
          </Link>
        </div>

        <div className="flex items-center space-x-3">
          {/* Quick Launch Button to GPUI WASM App */}
          <Link
            href="/app"
            className="inline-flex items-center gap-2 px-3.5 py-1.5 bg-[#780000] hover:bg-[#c1121f] text-white text-xs font-bold shadow-sm transition"
          >
            <span>{openAppLabel}</span>
            <ArrowRight className="w-3.5 h-3.5" />
          </Link>

          {/* Language Switcher */}
          <Link
            href={targetPath}
            className="inline-flex items-center space-x-1.5 px-3 py-1.5 border border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600 bg-slate-50 dark:bg-slate-800/80 text-xs font-bold text-[#003049] dark:text-[#fdf0d5] transition"
            title={`Switch to ${otherLocaleLabel}`}
          >
            <Globe className="w-3.5 h-3.5 text-[#669bbc]" />
            <span>{otherLocaleLabel}</span>
          </Link>

          {/* Theme Toggle */}
          <button
            onClick={toggleTheme}
            className="p-2 border border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600 bg-slate-50 dark:bg-slate-800/80 text-slate-600 dark:text-slate-300 transition"
            aria-label="Toggle theme"
          >
            {theme === "dark" ? (
              <Sun className="w-4 h-4 text-[#fdf0d5]" />
            ) : (
              <Moon className="w-4 h-4 text-[#003049]" />
            )}
          </button>
        </div>
      </div>
    </header>
  );
};
