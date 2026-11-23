"use client";

import React, { useState, use } from "react";
import { Landmark, Lock, Plus } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";
import { TransactionModal } from "@/components/TransactionModal";

export default function TransactionsPage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = use(params);
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  const { scope, transactions } = useLedger();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [search, setSearch] = useState("");
  const [filterType, setFilterType] = useState("all");

  const displayed = transactions
    .filter((tx) => (scope === "personal" ? tx.scope === "personal" : true))
    .filter((tx) => (filterType === "all" ? true : tx.txType === filterType))
    .filter((tx) => {
      if (!search) return true;
      const q = search.toLowerCase();
      return (
        tx.description.toLowerCase().includes(q) ||
        tx.category.toLowerCase().includes(q) ||
        tx.walletName.toLowerCase().includes(q)
      );
    });

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <div className="inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-extrabold uppercase bg-brand-cream/80 dark:bg-slate-800 text-brand-dark dark:text-brand-cream mb-1">
            {scope === "family" ? (
              <>
                <Landmark className="w-3 h-3 mr-1" />
                <span>{isRw ? "Umuryango" : "Family"}</span>
              </>
            ) : (
              <>
                <Lock className="w-3 h-3 mr-1" />
                <span>{isRw ? "Imari Bwite" : "Personal"}</span>
              </>
            )}
          </div>
          <h1 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight">
            {isRw ? "Urutonde rw'Imikoreshereze" : "Transaction Ledger"}
          </h1>
          <p className="text-slate-500 dark:text-slate-400 text-sm mt-1">
            {isRw
              ? "Imikoreshereze yose yanditse ku buryo bwizewe n'ihererekanya ry'umutungo"
              : "Complete transparent log of incomes, expenses, and custody handovers"}
          </p>
        </div>

        <button
          type="button"
          onClick={() => setIsModalOpen(true)}
          className="px-4 py-2 bg-brand-dark dark:bg-brand-active-crimson text-white font-bold text-xs rounded-xl shadow-md transition flex items-center space-x-1.5"
        >
          <Plus className="w-4 h-4 mr-1" />
          <span>{isRw ? "Andika Igikorwa Gishya" : "Record Transaction"}</span>
        </button>
      </div>

      <div className="bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 overflow-hidden shadow-sm transition-colors">
        <div className="p-4 border-b border-slate-100 dark:border-slate-800 flex flex-col sm:flex-row justify-between gap-3">
          <input
            type="text"
            placeholder={isRw ? "Shakisha..." : "Search transactions..."}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="px-3 py-1.5 text-xs rounded-xl border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none"
          />

          <select
            value={filterType}
            onChange={(e) => setFilterType(e.target.value)}
            className="px-3 py-1.5 text-xs rounded-xl border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none"
          >
            <option value="all">{isRw ? "Byose" : "All"}</option>
            <option value="income">{isRw ? "Ibyinjira" : "Income"}</option>
            <option value="expense">{isRw ? "Ibisohoka" : "Expense"}</option>
            <option value="handover">{isRw ? "Ihererekanya" : "Handover"}</option>
          </select>
        </div>

        <table className="w-full text-left text-xs">
          <thead className="bg-slate-50 dark:bg-slate-800/60 border-b border-slate-200 dark:border-slate-700 text-slate-500 dark:text-slate-400 font-semibold uppercase">
            <tr>
              <th className="px-4 py-3">{isRw ? "Itariki" : "Date"}</th>
              <th className="px-4 py-3">{isRw ? "Ibisobanuro" : "Description"}</th>
              <th className="px-4 py-3">{isRw ? "Icyiciro" : "Category"}</th>
              <th className="px-4 py-3">{isRw ? "Konti / Umubikizi" : "Wallet / Custodian"}</th>
              <th className="px-4 py-3">{isRw ? "Uwabikoze" : "Actor"}</th>
              <th className="px-4 py-3 text-right">{isRw ? "Amafaranga" : "Amount"}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-100 dark:divide-slate-800 text-slate-800 dark:text-slate-200">
            {displayed.map((tx) => (
              <tr
                key={tx.id}
                className="hover:bg-slate-50/50 dark:hover:bg-slate-800/40 transition"
              >
                <td className="px-4 py-3 text-slate-500 whitespace-nowrap">{tx.occurredAt}</td>
                <td className="px-4 py-3 font-semibold text-slate-900 dark:text-white">
                  {tx.description}
                </td>
                <td className="px-4 py-3">
                  <span className="px-2 py-0.5 rounded-full text-[10px] font-bold bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300">
                    {tx.category}
                  </span>
                </td>
                <td className="px-4 py-3">
                  {tx.txType === "handover"
                    ? `${tx.walletName} → ${tx.toWalletName}`
                    : tx.walletName}
                </td>
                <td className="px-4 py-3 text-slate-500">{tx.actorName}</td>
                <td
                  className={`px-4 py-3 text-right font-black ${
                    tx.txType === "income"
                      ? "text-emerald-600 dark:text-emerald-400"
                      : tx.txType === "expense"
                        ? "text-brand-crimson dark:text-rose-400"
                        : "text-brand-steel-blue"
                  }`}
                >
                  {tx.txType === "income" ? "+" : tx.txType === "expense" ? "-" : "↔ "}
                  {tx.amount.toLocaleString()} {tx.currency}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <TransactionModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        locale={locale}
      />
    </div>
  );
}
