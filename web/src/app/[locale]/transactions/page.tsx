export default async function TransactionsPage({
  params,
}: {
  params: Promise<{ locale: string }>;
}) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  const transactions = [
    {
      id: "tx-1",
      date: "2026-11-20",
      description: isRw
        ? "Umusanzu w'ubutabazi - Umwana ararwaye"
        : "Emergency Contribution - Medical",
      category: isRw ? "Ubutabazi" : "Emergency",
      amount: "-150,000 RWF",
      type: "expense",
      wallet: "MoMo (Keza Clarisse)",
    },
    {
      id: "tx-2",
      date: "2026-11-18",
      description: isRw ? "Ihererekanya: MoMo rugana muri Cash" : "Custody Handover: MoMo to Cash",
      category: isRw ? "Ihererekanya" : "Handover",
      amount: "~100,000 RWF",
      type: "handover",
      wallet: "Cash (Mugisha Jean)",
    },
    {
      id: "tx-3",
      date: "2026-11-15",
      description: isRw ? "Inyungu z'isarura ry'ikawa" : "Coffee harvest yield proceeds",
      category: isRw ? "Inyungu" : "Income",
      amount: "+800,000 RWF",
      type: "income",
      wallet: "Bank (BK Account)",
    },
  ];

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <h1 className="text-3xl font-black text-slate-900 tracking-tight">
            {isRw ? "Urutonde rw'Imikoreshereze" : "Transaction Ledger"}
          </h1>
          <p className="text-slate-500 text-sm mt-1">
            {isRw
              ? "Imikoreshereze yose yanditse ku buryo bwizewe"
              : "Complete transparent log of incomes, expenses, and custody handovers"}
          </p>
        </div>

        <button
          type="button"
          className="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-sm rounded-xl shadow-sm transition"
        >
          {isRw ? "+ Andika Igikorwa Gishya" : "+ Record Transaction"}
        </button>
      </div>

      <div className="bg-white rounded-2xl border border-slate-200 overflow-hidden shadow-sm">
        <table className="w-full text-left text-sm">
          <thead className="bg-slate-50 border-b border-slate-200 text-xs font-semibold uppercase text-slate-500">
            <tr>
              <th className="px-6 py-3">{isRw ? "Itariki" : "Date"}</th>
              <th className="px-6 py-3">{isRw ? "Ibisobanuro" : "Description"}</th>
              <th className="px-6 py-3">{isRw ? "Icyiciro" : "Category"}</th>
              <th className="px-6 py-3">{isRw ? "Konti / Umubikizi" : "Wallet / Custodian"}</th>
              <th className="px-6 py-3 text-right">{isRw ? "Amafaranga" : "Amount"}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-100">
            {transactions.map((tx) => (
              <tr key={tx.id} className="hover:bg-slate-50/50 transition">
                <td className="px-6 py-4 text-slate-500 whitespace-nowrap">{tx.date}</td>
                <td className="px-6 py-4 font-medium text-slate-900">{tx.description}</td>
                <td className="px-6 py-4">
                  <span className="px-2.5 py-1 rounded-full text-xs font-medium bg-slate-100 text-slate-700">
                    {tx.category}
                  </span>
                </td>
                <td className="px-6 py-4 text-slate-600">{tx.wallet}</td>
                <td
                  className={`px-6 py-4 text-right font-bold ${
                    tx.type === "income"
                      ? "text-emerald-600"
                      : tx.type === "expense"
                        ? "text-rose-600"
                        : "text-amber-600"
                  }`}
                >
                  {tx.amount}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
