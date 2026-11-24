import Link from "next/link";
import { Smartphone, Shield, HeartHandshake, ArrowRight } from "lucide-react";
import { t } from "@/lib/i18n";

export default async function HomePage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const title = t(locale, "landing.title");
  const subtitle = t(locale, "landing.subtitle");
  const openApp = locale === "rw" ? "Fungura Porogaramu (Open App)" : "Open App";
  const viewReportDemo = t(locale, "landing.view_report_demo");

  return (
    <div className="py-12 space-y-12">
      <div className="text-center max-w-3xl mx-auto space-y-6">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 bg-[#fdf0d5] text-[#003049] border border-[#fdf0d5] text-xs font-bold shadow-sm">
          <span>{t(locale, "landing.tagline")}</span>
        </div>
        <h1 className="text-4xl sm:text-6xl font-black text-[#003049] dark:text-white tracking-tight leading-tight">
          {title}
        </h1>
        <p className="text-base sm:text-lg text-slate-600 dark:text-slate-300 leading-relaxed font-normal">
          {subtitle}
        </p>

        <div className="pt-4 flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href="/app"
            className="inline-flex items-center justify-center gap-2 px-7 py-3.5 bg-[#780000] hover:bg-[#c1121f] text-white font-extrabold shadow-lg transition transform hover:-translate-y-0.5"
          >
            <span>{openApp}</span>
            <ArrowRight className="w-4 h-4" />
          </Link>
          <Link
            href={`/${locale}/report/family-november-2026`}
            className="px-7 py-3.5 bg-white dark:bg-slate-800 hover:bg-[#fdf0d5]/40 border border-[#669bbc]/40 text-[#003049] dark:text-[#fdf0d5] font-extrabold shadow-sm transition"
          >
            {viewReportDemo}
          </Link>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-[#003049] p-7 border border-slate-200 dark:border-slate-800 shadow-sm transition">
          <div className="w-12 h-12 bg-[#fdf0d5]/60 dark:bg-slate-800 flex items-center justify-center mb-4 text-[#669bbc]">
            <Smartphone className="w-6 h-6" />
          </div>
          <h3 className="font-bold text-lg text-slate-900 dark:text-white">
            {t(locale, "landing.feature_custody_title")}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 mt-2 leading-relaxed">
            {t(locale, "landing.feature_custody_desc")}
          </p>
        </div>

        <div className="bg-white dark:bg-[#003049] p-7 border border-slate-200 dark:border-slate-800 shadow-sm transition">
          <div className="w-12 h-12 bg-[#fdf0d5]/60 dark:bg-slate-800 flex items-center justify-center mb-4 text-[#c1121f]">
            <Shield className="w-6 h-6" />
          </div>
          <h3 className="font-bold text-lg text-slate-900 dark:text-white">
            {t(locale, "landing.feature_scopes_title")}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 mt-2 leading-relaxed">
            {t(locale, "landing.feature_scopes_desc")}
          </p>
        </div>

        <div className="bg-white dark:bg-[#003049] p-7 border border-slate-200 dark:border-slate-800 shadow-sm transition">
          <div className="w-12 h-12 bg-[#fdf0d5]/60 dark:bg-slate-800 flex items-center justify-center mb-4 text-[#669bbc]">
            <HeartHandshake className="w-6 h-6" />
          </div>
          <h3 className="font-bold text-lg text-slate-900 dark:text-white">
            {t(locale, "landing.feature_elders_title")}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 mt-2 leading-relaxed">
            {t(locale, "landing.feature_elders_desc")}
          </p>
        </div>
      </div>
    </div>
  );
}
