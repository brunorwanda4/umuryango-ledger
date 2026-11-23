"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { ScopeSwitcher } from "@/components/ScopeSwitcher";
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

  const navDashboard = locale === "rw" ? "Incamake" : "Dashboard";
  const navTransactions = locale === "rw" ? "Imikoreshereze" : "Transactions";
  const navProjects = locale === "rw" ? "Imishinga" : "Projects";

  return (
    <header className="sticky top-0 z-40 bg-white/95 dark:bg-brand-dark/95 backdrop-blur border-b border-slate-200 dark:border-slate-800 transition-colors no-print">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <div className="flex items-center space-x-6">
          <Link href={`/${locale}`} className="flex items-center space-x-3 group">
            <img
              src="/icon-192.png"
              alt="Umuryango Ledger Emblem"
              width={36}
              height={36}
              className="rounded-xl shadow-sm border border-slate-200 dark:border-slate-700 group-hover:scale-105 transition-transform"
            />
            <div className="flex flex-col">
              <span className="text-lg font-black tracking-tight text-brand-dark dark:text-brand-cream leading-none">
                Umuryango <span className="text-brand-active-crimson">Ledger</span>
              </span>
              <span className="text-[10px] font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-widest mt-0.5">
                Bilingual Accountability
              </span>
            </div>
          </Link>

          <nav className="hidden md:flex space-x-1 text-sm font-semibold">
            <Link
              href={`/${locale}/dashboard`}
              className={`px-3 py-1.5 rounded-lg transition ${
                pathname.includes("/dashboard")
                  ? "bg-slate-100 dark:bg-slate-800 text-brand-active-crimson font-bold"
                  : "text-slate-600 dark:text-slate-300 hover:text-brand-active-crimson"
              }`}
            >
              {navDashboard}
            </Link>
            <Link
              href={`/${locale}/transactions`}
              className={`px-3 py-1.5 rounded-lg transition ${
                pathname.includes("/transactions")
                  ? "bg-slate-100 dark:bg-slate-800 text-brand-active-crimson font-bold"
                  : "text-slate-600 dark:text-slate-300 hover:text-brand-active-crimson"
              }`}
            >
              {navTransactions}
            </Link>
            <Link
              href={`/${locale}/projects`}
              className={`px-3 py-1.5 rounded-lg transition ${
                pathname.includes("/projects")
                  ? "bg-slate-100 dark:bg-slate-800 text-brand-active-crimson font-bold"
                  : "text-slate-600 dark:text-slate-300 hover:text-brand-active-crimson"
              }`}
            >
              {navProjects}
            </Link>
          </nav>
        </div>

        <div className="flex items-center space-x-3">
          <ScopeSwitcher locale={locale} />

          <button
            type="button"
            onClick={toggleTheme}
            title={theme === "dark" ? "Light Mode" : "Dark Mode"}
            className="p-2 rounded-xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-600 dark:text-brand-cream hover:bg-slate-100 dark:hover:bg-slate-700 transition"
          >
            {theme === "dark" ? "☀️" : "🌙"}
          </button>

          <Link
            href={targetPath}
            className="px-2.5 py-1.5 rounded-xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 text-xs font-bold text-slate-700 dark:text-slate-200 hover:border-brand-active-crimson transition shadow-sm"
          >
            🌐 {otherLocaleLabel}
          </Link>
        </div>
      </div>
    </header>
  );
};
