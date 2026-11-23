import React from "react";
import Link from "next/link";
import { ScopeSwitcher } from "@/components/ScopeSwitcher";

export const metadata = {
  title: "Umuryango Ledger - Family & Personal Financial Accountability",
  description: "Multi-tenant bilingual family and personal finance accountability platform.",
  icons: {
    icon: "/favicon.ico",
    shortcut: "/favicon.ico",
    apple: "/apple-touch-icon.png",
  },
  manifest: "/manifest.json",
  themeColor: "#003049",
};

export default async function LocaleLayout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ locale: string }>;
}) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const otherLocale = locale === "rw" ? "en" : "rw";
  const otherLocaleLabel = locale === "rw" ? "English" : "Kinyarwanda";

  const navDashboard = locale === "rw" ? "Incamake" : "Dashboard";
  const navTransactions = locale === "rw" ? "Imikoreshereze" : "Transactions";
  const navProjects = locale === "rw" ? "Imishinga" : "Projects";

  return (
    <html lang={locale}>
      <body className="min-h-screen bg-slate-50 text-slate-900 font-sans antialiased">
        <header className="sticky top-0 z-40 bg-white/95 backdrop-blur border-b border-slate-200">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
            <div className="flex items-center space-x-6">
              <Link href={`/${locale}`} className="flex items-center space-x-3">
                <img
                  src="/icon-192.png"
                  alt="Umuryango Ledger Emblem"
                  width={34}
                  height={34}
                  className="rounded-lg shadow-sm border border-slate-200"
                />
                <span className="text-xl font-bold tracking-tight text-slate-900">
                  Umuryango <span className="text-emerald-700">Ledger</span>
                </span>
              </Link>
              <nav className="hidden md:flex space-x-4 text-sm font-medium text-slate-600">
                <Link href={`/${locale}/dashboard`} className="hover:text-emerald-700 transition">
                  {navDashboard}
                </Link>
                <Link
                  href={`/${locale}/transactions`}
                  className="hover:text-emerald-700 transition"
                >
                  {navTransactions}
                </Link>
                <Link href={`/${locale}/projects`} className="hover:text-emerald-700 transition">
                  {navProjects}
                </Link>
              </nav>
            </div>

            <div className="flex items-center space-x-4">
              <ScopeSwitcher locale={locale} />
              <Link
                href={`/${otherLocale}`}
                className="text-xs font-semibold px-2.5 py-1.5 rounded-md border border-slate-300 hover:bg-slate-100 transition"
              >
                🌐 {otherLocaleLabel}
              </Link>
            </div>
          </div>
        </header>

        <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">{children}</main>
      </body>
    </html>
  );
}
