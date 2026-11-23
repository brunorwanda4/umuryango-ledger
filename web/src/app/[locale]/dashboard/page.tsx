"use client";

import React, { useState, use } from "react";
import Link from "next/link";
import {
  Landmark,
  Lock,
  Handshake,
  Plus,
  Building2,
  Smartphone,
  Banknote,
  ShieldCheck,
  ArrowRight,
} from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";
import { TransactionModal } from "@/components/TransactionModal";
import { HandoverModal } from "@/components/HandoverModal";
import { CustodyChart } from "@/components/CustodyChart";

export default function DashboardPage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = use(params);
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  const { scope, wallets, transactions, kpis } = useLedger();

  const [isTxModalOpen, setIsTxModalOpen] = useState(false);
  const [isHandoverModalOpen, setIsHandoverModalOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [filterType, setFilterType] = useState<string>("all");

  const filteredTransactions = transactions
    .filter((tx) => (scope === "personal" ? tx.scope === "personal" : true))
    .filter((tx) => {
      if (filterType === "all") return true;
      return tx.txType === filterType;
    })
    .filter((tx) => {
      if (!searchQuery) return true;
      const q = searchQuery.toLowerCase();
      return (
        tx.description.toLowerCase().includes(q) ||
        tx.category.toLowerCase().includes(q) ||
        tx.walletName.toLowerCase().includes(q) ||
        tx.actorName.toLowerCase().includes(q)
      );
    });

  return (
    <div className="space-y-8">
      {/* Page Header with Action Buttons */}
      <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
        <div>
          <div className="flex items-center space-x-2">
            <span
              className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-[11px] font-extrabold uppercase tracking-wider ${
                scope === "family"
                  ? "bg-brand-dark text-brand-cream"
                  : "bg-brand-crimson text-white"
              }`}
            >
              {scope === "family" ? (
                <>
                  <Landmark className="w-3 h-3 mr-1" />
                  <span>{isRw ? "Igitabo cy'Umuryango" : "Family Scope"}</span>
                </>
              ) : (
                <>
                  <Lock className="w-3 h-3 mr-1" />
                  <span>{isRw ? "Imari Yanjye Bwite" : "Personal Scope"}</span>
                </>
              )}
            </span>
            <span className="text-xs text-slate-400">•</span>
            <span className="text-xs text-slate-500 dark:text-slate-400">
              {isRw ? "Igihe: Ugushyingo 2026" : "Period: November 2026"}
            </span>
          </div>
          <h1 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight mt-1">
            {isRw ? "Incamake y'Imari" : "Financial Dashboard"}
          </h1>
          <p className="text-slate-500 dark:text-slate-400 text-sm mt-0.5">
            {scope === "family"
              ? isRw
                ? "Irebere uko umutungo w'umuryango wifashe muri rusange n'abawubitse"
                : "Real-time multi-tenant view of custody wallets, custodians, and handovers"
              : isRw
                ? "Konti zawe bwite zitarangirizwa mu muryango"
                : "Isolated personal finances and private expense tracking"}
          </p>
        </div>

        <div className="flex items-center space-x-3">
          {scope === "family" && (
            <button
              type="button"
              onClick={() => setIsHandoverModalOpen(true)}
              className="px-4 py-2.5 rounded-xl border border-brand-dark dark:border-slate-700 bg-white dark:bg-slate-800 text-brand-dark dark:text-brand-cream hover:bg-slate-50 dark:hover:bg-slate-700 font-bold text-xs shadow-sm transition flex items-center space-x-1.5"
            >
              <Handshake className="w-4 h-4 mr-1 text-brand-dark dark:text-brand-cream" />
              <span>{isRw ? "+ Guhererekanya" : "+ Handover Custody"}</span>
            </button>
          )}

          <button
            type="button"
            onClick={() => setIsTxModalOpen(true)}
            className="px-5 py-2.5 rounded-xl bg-brand-dark dark:bg-brand-active-crimson text-white font-bold text-xs shadow-md hover:opacity-95 transition flex items-center space-x-1.5"
          >
            <Plus className="w-4 h-4 mr-1" />
            <span>{isRw ? "+ Andika Igikorwa" : "+ Log Transaction"}</span>
          </button>
        </div>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="p-5 bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm transition-colors">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
            {isRw ? "Umutungo Wose" : "Total Balance"}
          </span>
          <div className="text-2xl font-black text-brand-dark dark:text-brand-cream mt-2">
            {kpis.totalAssets.toLocaleString()} RWF
          </div>
          <span className="text-xs text-brand-steel-blue font-semibold mt-1 block">
            ↑ +450,000 RWF ({isRw ? "uku kwezi" : "this month"})
          </span>
        </div>

        <div className="p-5 bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm transition-colors">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
            {isRw ? "Mobile Money (MoMo)" : "MoMo Custody"}
          </span>
          <div className="text-2xl font-black text-[#e09f3e] mt-2">
            {kpis.momoTotal.toLocaleString()} RWF
          </div>
          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">
            {isRw ? "Umubikizi: Keza Clarisse (Mama)" : "Custodian: Keza Clarisse (Mama)"}
          </span>
        </div>

        <div className="p-5 bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm transition-colors">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
            {isRw ? "Amafaranga mu Ntoki" : "Cash in Hand"}
          </span>
          <div className="text-2xl font-black text-brand-active-crimson mt-2">
            {kpis.cashInHand.toLocaleString()} RWF
          </div>
          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">
            {isRw ? "Imbaho z'ababyeyi (Mama & Papa)" : "Held by Mama & Papa"}
          </span>
        </div>

        <div className="p-5 bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm transition-colors">
          <span className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
            {isRw ? "Amadeni n'Inguzanyo" : "Outstanding Debts"}
          </span>
          <div className="text-2xl font-black text-brand-crimson dark:text-rose-400 mt-2">
            {kpis.outstandingDebts.toLocaleString()} RWF
          </div>
          <span className="text-xs text-slate-500 dark:text-slate-400 mt-1 block">
            {isRw ? "Igihe cyo kwishyura: 15/12" : "Due: Dec 15"}
          </span>
        </div>
      </div>

      {/* Custody Chart Visualization */}
      <CustodyChart locale={locale} />

      {/* Wallets Overview */}
      <div className="space-y-3">
        <h3 className="text-lg font-bold text-slate-900 dark:text-white">
          {isRw ? "Konti z'Umutungo & Abawubitse" : "Custody Wallets & Custodians"}
        </h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {wallets.map((wallet) => (
            <div
              key={wallet.id}
              className="p-4 bg-white dark:bg-brand-dark rounded-xl border border-slate-200 dark:border-slate-800 shadow-sm hover:border-brand-steel-blue transition"
            >
              <div className="flex items-center justify-between">
                <span className="p-2 rounded-lg bg-slate-100 dark:bg-slate-800">
                  {wallet.type === "bank" ? (
                    <Building2 className="w-5 h-5 text-slate-600 dark:text-slate-300" />
                  ) : wallet.type === "momo" ? (
                    <Smartphone className="w-5 h-5 text-amber-600 dark:text-amber-400" />
                  ) : (
                    <Banknote className="w-5 h-5 text-brand-crimson dark:text-rose-400" />
                  )}
                </span>
                <span className="text-[10px] font-bold px-2 py-0.5 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300">
                  {wallet.custodianName}
                </span>
              </div>
              <h4 className="font-bold text-slate-900 dark:text-white text-sm mt-2">
                {wallet.name}
              </h4>
              <div className="text-lg font-extrabold text-brand-dark dark:text-brand-cream mt-1">
                {wallet.balance.toLocaleString()} {wallet.currency}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Transactions List with Search & Filter */}
      <div className="bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden transition-colors">
        <div className="p-5 border-b border-slate-100 dark:border-slate-800 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div>
            <h3 className="text-base font-bold text-slate-900 dark:text-white">
              {isRw ? "Ibyakozwe vuba" : "Recent Ledger Activity"}
            </h3>
            <p className="text-xs text-slate-500 dark:text-slate-400">
              {isRw
                ? "Urutonde rw'ibyinjiye, ibyasohotse, n'ihererekanya"
                : "Live history of inflows, outflows, and custody transfers"}
            </p>
          </div>

          <div className="flex items-center space-x-2">
            <input
              type="text"
              placeholder={isRw ? "Shakisha..." : "Search transactions..."}
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="px-3 py-1.5 text-xs rounded-xl border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-steel-blue"
            />

            <select
              value={filterType}
              onChange={(e) => setFilterType(e.target.value)}
              className="px-3 py-1.5 text-xs rounded-xl border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none"
            >
              <option value="all">{isRw ? "Byose" : "All"}</option>
              <option value="income">{isRw ? "Ibyinjira" : "Income"}</option>
              <option value="expense">{isRw ? "Ibisohoka" : "Expenses"}</option>
              <option value="handover">{isRw ? "Ihererekanya" : "Handovers"}</option>
            </select>
          </div>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full text-left text-xs">
            <thead className="bg-slate-50 dark:bg-slate-800/60 text-slate-500 dark:text-slate-400 uppercase font-semibold">
              <tr>
                <th className="py-3 px-4">{isRw ? "Ubwoko" : "Type"}</th>
                <th className="py-3 px-4">{isRw ? "Ibisobanuro" : "Description"}</th>
                <th className="py-3 px-4">{isRw ? "Konti" : "Wallet"}</th>
                <th className="py-3 px-4">{isRw ? "Uwabikoze" : "Actor"}</th>
                <th className="py-3 px-4">{isRw ? "Itariki" : "Date"}</th>
                <th className="py-3 px-4 text-right">{isRw ? "Amafaranga" : "Amount"}</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-100 dark:divide-slate-800 text-slate-800 dark:text-slate-200">
              {filteredTransactions.length === 0 ? (
                <tr>
                  <td colSpan={6} className="py-8 text-center text-slate-400">
                    {isRw ? "Nta bikorwa bibonetse" : "No transactions found"}
                  </td>
                </tr>
              ) : (
                filteredTransactions.map((tx) => (
                  <tr
                    key={tx.id}
                    className="hover:bg-slate-50/50 dark:hover:bg-slate-800/40 transition"
                  >
                    <td className="py-3 px-4">
                      <span
                        className={`inline-block px-2 py-0.5 rounded-full font-bold text-[10px] ${
                          tx.txType === "income"
                            ? "bg-emerald-100 text-emerald-800 dark:bg-emerald-900/40 dark:text-emerald-300"
                            : tx.txType === "expense"
                              ? "bg-rose-100 text-rose-800 dark:bg-rose-900/40 dark:text-rose-300"
                              : "bg-blue-100 text-blue-800 dark:bg-blue-900/40 dark:text-blue-300"
                        }`}
                      >
                        {tx.txType === "income"
                          ? isRw
                            ? "Ibyinjira"
                            : "Income"
                          : tx.txType === "expense"
                            ? isRw
                              ? "Ibisohoka"
                              : "Expense"
                            : isRw
                              ? "Ihererekanya"
                              : "Handover"}
                      </span>
                    </td>
                    <td className="py-3 px-4">
                      <div className="font-semibold text-slate-900 dark:text-white">
                        {tx.description}
                      </div>
                      <div className="text-[10px] text-slate-400">{tx.category}</div>
                    </td>
                    <td className="py-3 px-4">
                      {tx.txType === "handover" ? (
                        <span>
                          {tx.walletName} → <strong>{tx.toWalletName}</strong>
                        </span>
                      ) : (
                        tx.walletName
                      )}
                    </td>
                    <td className="py-3 px-4 text-slate-600 dark:text-slate-400">{tx.actorName}</td>
                    <td className="py-3 px-4 text-slate-400">{tx.occurredAt}</td>
                    <td
                      className={`py-3 px-4 text-right font-black ${
                        tx.txType === "income"
                          ? "text-emerald-700 dark:text-emerald-400"
                          : tx.txType === "expense"
                            ? "text-brand-crimson dark:text-rose-400"
                            : "text-slate-700 dark:text-slate-300"
                      }`}
                    >
                      {tx.txType === "income" ? "+" : tx.txType === "expense" ? "-" : "↔ "}
                      {tx.amount.toLocaleString()} {tx.currency}
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Zero-Login Mom's Report Card Link */}
      <div className="p-6 rounded-2xl bg-gradient-to-r from-brand-dark to-[#054468] text-white flex flex-col md:flex-row items-start md:items-center justify-between gap-6 shadow-xl border border-white/10">
        <div className="flex items-center space-x-4">
          <img
            src="/icon-192.png"
            alt="Umuryango Emblem"
            width={52}
            height={52}
            className="rounded-2xl shadow-lg border border-white/20"
          />
          <div>
            <span className="inline-flex items-center text-[10px] font-bold uppercase tracking-widest text-brand-cream/80 bg-white/10 px-2.5 py-0.5 rounded-full mb-1">
              <ShieldCheck className="w-3 h-3 mr-1 text-brand-cream" />
              <span>{isRw ? "Umutekano Udakeneye Ijambobanga" : "Zero-Login Magic Link"}</span>
            </span>
            <h3 className="text-xl font-bold tracking-tight">
              {isRw ? "Raporo y'Ababyeyi n'Abakambwe (Mother's Report)" : "Mother's Summary Report"}
            </h3>
            <p className="text-xs text-brand-cream/90 mt-1 max-w-xl">
              {isRw
                ? "Fungura cyangwa usangize ababyeyi raporo y'imari yoroheje cyane kureba nta kwijinjiza bisabwa (No login needed)."
                : "Open or share a simplified, high-trust elder-friendly financial summary view requiring zero authentication."}
            </p>
          </div>
        </div>

        <Link
          href={`/${locale}/report/family-november-2026`}
          className="inline-flex items-center px-5 py-2.5 rounded-xl bg-brand-cream text-brand-dark hover:bg-white font-extrabold text-xs shadow-md transition whitespace-nowrap"
        >
          <span>{isRw ? "Fungura Raporo y'Ababyeyi" : "Open Mother's Report"}</span>
          <ArrowRight className="w-4 h-4 ml-1.5" />
        </Link>
      </div>

      {/* Modals */}
      <TransactionModal
        isOpen={isTxModalOpen}
        onClose={() => setIsTxModalOpen(false)}
        locale={locale}
      />
      <HandoverModal
        isOpen={isHandoverModalOpen}
        onClose={() => setIsHandoverModalOpen(false)}
        locale={locale}
      />
    </div>
  );
}
