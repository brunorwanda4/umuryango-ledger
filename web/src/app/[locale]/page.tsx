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
  const viewReportDemo = isRw ? "Kureba Raporo y'Icyitegererezo" : "View Demo Report";

  return (
    <div className="py-12">
      <div className="text-center max-w-3xl mx-auto space-y-6">
        <div className="inline-flex items-center gap-2 px-3 py-1 bg-emerald-100 text-emerald-800 rounded-full text-xs font-semibold">
          🇷🇼 Made for Rwandan Families & Diaspora
        </div>
        <h1 className="text-4xl sm:text-5xl font-black text-slate-900 tracking-tight leading-tight">
          {title}
        </h1>
        <p className="text-lg text-slate-600 leading-relaxed">{subtitle}</p>

        <div className="pt-4 flex flex-col sm:flex-row gap-4 justify-center">
          <Link
            href={`/${locale}/dashboard`}
            className="px-6 py-3 rounded-xl bg-emerald-600 hover:bg-emerald-700 text-white font-bold shadow-lg shadow-emerald-600/20 transition"
          >
            {openDashboard}
          </Link>
          <Link
            href={`/${locale}/report/demo-token`}
            className="px-6 py-3 rounded-xl bg-white hover:bg-slate-50 border border-slate-300 text-slate-800 font-bold shadow-sm transition"
          >
            {viewReportDemo}
          </Link>
        </div>
      </div>

      <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8">
        <div className="bg-white p-6 rounded-2xl border border-slate-200 shadow-sm">
          <div className="text-3xl mb-3">📱</div>
          <h3 className="font-bold text-lg text-slate-900">
            {isRw ? "Amakonti n'Umutungo Ubitswe" : "Custody Wallets (MoMo/Bank/Cash)"}
          </h3>
          <p className="text-sm text-slate-600 mt-2">
            {isRw
              ? "Buri faranga ryose rigira umubikizi urifite mu ntoki cyangwa kuri telefoni, hamwe n'ihererekanya ryemezwa n'ababigiramo uruhare bombi."
              : "Every Franc is accounted for with an assigned custodian and dual-confirmation handovers."}
          </p>
        </div>

        <div className="bg-white p-6 rounded-2xl border border-slate-200 shadow-sm">
          <div className="text-3xl mb-3">🔒</div>
          <h3 className="font-bold text-lg text-slate-900">
            {isRw ? "Imari Bwite vs Iy'Umuryango" : "Personal vs Family Scopes"}
          </h3>
          <p className="text-sm text-slate-600 mt-2">
            {isRw
              ? "Icyuho cy'umutekano kibuza imari yawe bwite kuvangwa n'iy'umuryango binyuze muri Row-Level Security (RLS)."
              : "Strict architectural and database isolation ensures private finances are never mixed with shared family funds."}
          </p>
        </div>

        <div className="bg-white p-6 rounded-2xl border border-slate-200 shadow-sm">
          <div className="text-3xl mb-3">👵</div>
          <h3 className="font-bold text-lg text-slate-900">
            {isRw ? "Raporo y'Ababyeyi (Zero-Login)" : "Mother's Zero-Login Report"}
          </h3>
          <p className="text-sm text-slate-600 mt-2">
            {isRw
              ? "Ihuza ryihariye ryemerera ababyeyi n'abakambwe kureba uko umutungo w'umuryango wifashe batiriwe bafungura konti cyangwa bandika ijambobanga."
              : "Secure tokenized links give elders instant visibility into family balances without passwords or logins."}
          </p>
        </div>
      </div>
    </div>
  );
}
