import React from "react";
import { LedgerProvider } from "@/lib/LedgerContext";
import { HeaderNav } from "@/components/HeaderNav";
import { ToastContainer } from "@/components/ToastContainer";
import { AutoUpdateListener } from "@/components/AutoUpdateListener";

export default async function LocaleLayout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ locale: string }>;
}) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";

  return (
    <LedgerProvider currentLocale={locale}>
      <AutoUpdateListener locale={locale} />
      <HeaderNav locale={locale} />
      <ToastContainer />
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">{children}</main>
    </LedgerProvider>
  );
}
