import Link from "next/link";

export default async function DashboardPage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-black text-slate-900 tracking-tight">
          {isRw ? "Incamake y'Imari" : "Financial Dashboard"}
        </h1>
        <p className="text-slate-500 text-sm mt-1">
          {isRw
            ? "Irebere uko umutungo w'umuryango wifashe muri rusange"
            : "Overview of all active custody balances and family commitments"}
        </p>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="p-5 bg-white rounded-2xl border border-slate-200 shadow-sm">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500">
            {isRw ? "Umutungo Wose" : "Total Assets"}
          </span>
          <div className="text-2xl font-black text-emerald-700 mt-2">10,000,000 RWF</div>
          <span className="text-xs text-emerald-600 font-medium mt-1 block">
            ↑ +450,000 RWF (uku kwezi)
          </span>
        </div>

        <div className="p-5 bg-white rounded-2xl border border-slate-200 shadow-sm">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500">
            {isRw ? "Mobile Money (MoMo)" : "MoMo Custody"}
          </span>
          <div className="text-2xl font-black text-amber-600 mt-2">1,450,000 RWF</div>
          <span className="text-xs text-slate-500 mt-1 block">
            {isRw ? "Umubikizi: Keza Clarisse" : "Custodian: Keza Clarisse"}
          </span>
        </div>

        <div className="p-5 bg-white rounded-2xl border border-slate-200 shadow-sm">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500">
            {isRw ? "Amafaranga yo mu Ntoki" : "Cash in Hand"}
          </span>
          <div className="text-2xl font-black text-blue-600 mt-2">350,000 RWF</div>
          <span className="text-xs text-slate-500 mt-1 block">
            {isRw ? "Umubikizi: Mugisha Jean" : "Custodian: Mugisha Jean"}
          </span>
        </div>

        <div className="p-5 bg-white rounded-2xl border border-slate-200 shadow-sm">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500">
            {isRw ? "Amadeni n'Inguzanyo" : "Outstanding Debts"}
          </span>
          <div className="text-2xl font-black text-rose-600 mt-2">150,000 RWF</div>
          <span className="text-xs text-rose-500 mt-1 block">
            {isRw ? "Igihe cyo kwishyura: 15/12" : "Due: Dec 15"}
          </span>
        </div>
      </div>

      {/* Pending Handover Alert */}
      <div className="p-4 bg-amber-50 rounded-2xl border border-amber-200 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div className="flex items-center space-x-3">
          <span className="text-2xl">🤝</span>
          <div>
            <h4 className="font-bold text-amber-900 text-sm">
              {isRw ? "Ihererekanya ry'Umutungo rirategerejwe" : "Pending Custody Handover"}
            </h4>
            <p className="text-xs text-amber-800">
              {isRw
                ? "Mugisha Jean yohereje 100,000 RWF (Cash) kuri Gasana Eric. Birasaba kwemezwa."
                : "Mugisha Jean transferred 100,000 RWF (Cash) to Gasana Eric. Awaiting acknowledgment."}
            </p>
          </div>
        </div>
        <button
          type="button"
          className="px-4 py-2 bg-amber-600 hover:bg-amber-700 text-white font-bold text-xs rounded-xl transition"
        >
          {isRw ? "Emeza ko Wakiriye" : "Acknowledge Receipt"}
        </button>
      </div>

      {/* Quick Navigation Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="p-6 bg-white rounded-2xl border border-slate-200 shadow-sm">
          <h3 className="font-bold text-slate-900 mb-2">
            {isRw ? "Imishinga y'Umuryango Iri Gukorwa" : "Active Family Projects"}
          </h3>
          <p className="text-sm text-slate-500 mb-4">
            {isRw
              ? "Gusana inzu y'ababyeyi (Target: 5,000,000 RWF - 24% yabonetse)"
              : "House Renovation (Target: 5,000,000 RWF - 24% achieved)"}
          </p>
          <Link
            href={`/${locale}/projects`}
            className="text-xs font-semibold text-emerald-700 hover:text-emerald-800"
          >
            {isRw ? "Reba Imishinga Yose →" : "View All Projects →"}
          </Link>
        </div>

        <div className="p-6 bg-white rounded-2xl border border-slate-200 shadow-sm">
          <h3 className="font-bold text-slate-900 mb-2">
            {isRw ? "Raporo y'Umuryango (Zero-Login)" : "Zero-Login Report Sharing"}
          </h3>
          <p className="text-sm text-slate-500 mb-4">
            {isRw
              ? "Sangiza ababyeyi n'abakambwe umurongo wo kureba incamake y'imari idasaba ijambobanga."
              : "Generate secure token links for parents and family elders to review finances."}
          </p>
          <Link
            href={`/${locale}/report/demo-token`}
            className="text-xs font-semibold text-emerald-700 hover:text-emerald-800"
          >
            {isRw ? "Fungura Raporo y'Icyitegererezo →" : "Open Sample Report →"}
          </Link>
        </div>
      </div>
    </div>
  );
}
