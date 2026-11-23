import Link from "next/link";

export default async function HomePage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  const title = isRw
    ? "Imicungire n'Ibaruramari ry'Umuryango n'Umuntu ku Giti Cye"
    : "Family & Personal Financial Accountability";
  const subtitle = isRw
    ? "Igitabo gifasha imiryango gucunga umutungo ubitswe (MoMo, Banki, Amafaranga yo mu ntoki), gukorera hamwe imishinga, no guha ababyeyi raporo zizewe zidakeneye kwinjira (Zero-Login)."
    : "A modern platform empowering families to track custody assets (MoMo, Bank, Cash), execute shared projects transparently, and generate zero-login reports for elders.";
  const openDashboard = isRw ? "Kwinjira mu Incamake" : "Open Dashboard";
  const viewReportDemo = isRw ? "Kureba Raporo y'Ababyeyi" : "View Mother's Report";

  return (
    <div className="py-12 space-y-12">
      <div className="text-center max-w-3xl mx-auto space-y-6">
        <div className="inline-flex items-center gap-2 px-3.5 py-1.5 bg-brand-cream/80 dark:bg-slate-800 text-brand-dark dark:text-brand-cream border border-brand-cream dark:border-slate-700 rounded-full text-xs font-bold shadow-sm">
          🇷🇼 Made for Rwandan Families & Diaspora
        </div>
        <h1 className="text-4xl sm:text-6xl font-black text-brand-dark dark:text-white tracking-tight leading-tight">
          {title}
        </h1>
        <p className="text-base sm:text-lg text-slate-600 dark:text-slate-300 leading-relaxed font-normal">
          {subtitle}
        </p>

        <div className="pt-4 flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href={`/${locale}/dashboard`}
            className="px-7 py-3.5 rounded-2xl bg-brand-dark dark:bg-brand-active-crimson text-white font-extrabold shadow-lg hover:opacity-95 transition transform hover:-translate-y-0.5"
          >
            {openDashboard} →
          </Link>
          <Link
            href={`/${locale}/report/family-november-2026`}
            className="px-7 py-3.5 rounded-2xl bg-white dark:bg-slate-800 hover:bg-slate-50 dark:hover:bg-slate-700 border border-slate-300 dark:border-slate-700 text-brand-dark dark:text-brand-cream font-extrabold shadow-sm transition"
          >
            {viewReportDemo}
          </Link>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white dark:bg-brand-dark p-7 rounded-3xl border border-slate-200 dark:border-slate-800 shadow-sm transition">
          <div className="text-3xl mb-3">📱</div>
          <h3 className="font-bold text-lg text-slate-900 dark:text-white">
            {isRw ? "Amakonti n'Umutungo Ubitswe" : "Custody Wallets (MoMo/Bank/Cash)"}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 mt-2 leading-relaxed">
            {isRw
              ? "Buri faranga ryose rigira umubikizi urifite mu ntoki cyangwa kuri telefoni, hamwe n'ihererekanya ryemezwa n'ababigiramo uruhare bombi."
              : "Every Franc is accounted for with an assigned custodian and dual-confirmation handovers."}
          </p>
        </div>

        <div className="bg-white dark:bg-brand-dark p-7 rounded-3xl border border-slate-200 dark:border-slate-800 shadow-sm transition">
          <div className="text-3xl mb-3">🔒</div>
          <h3 className="font-bold text-lg text-slate-900 dark:text-white">
            {isRw ? "Imari Bwite vs Iy'Umuryango" : "Personal vs Family Scopes"}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 mt-2 leading-relaxed">
            {isRw
              ? "Icyuho cy'umutekano kibuza imari yawe bwite kuvangwa n'iy'umuryango binyuze muri Row-Level Security (RLS)."
              : "Strict architectural and database isolation ensures private finances are never mixed with shared family funds."}
          </p>
        </div>

        <div className="bg-white dark:bg-brand-dark p-7 rounded-3xl border border-slate-200 dark:border-slate-800 shadow-sm transition">
          <div className="text-3xl mb-3">👵</div>
          <h3 className="font-bold text-lg text-slate-900 dark:text-white">
            {isRw ? "Raporo y'Ababyeyi (Zero-Login)" : "Mother's Zero-Login Report"}
          </h3>
          <p className="text-sm text-slate-600 dark:text-slate-400 mt-2 leading-relaxed">
            {isRw
              ? "Ihuza ryihariye ryemerera ababyeyi n'abakambwe kureba uko umutungo w'umuryango wifashe batiriwe bafungura konti cyangwa bandika ijambobanga."
              : "Secure tokenized links give elders instant visibility into family balances without passwords or logins."}
          </p>
        </div>
      </div>
    </div>
  );
}
