"use client";

import React, { useState } from "react";
import { AlertTriangle, ArrowDownRight, ArrowUpRight, X } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";
import { t } from "@/lib/i18n";

interface TransactionModalProps {
  isOpen: boolean;
  onClose: () => void;
  locale: "rw" | "en";
}

export const TransactionModal: React.FC<TransactionModalProps> = ({ isOpen, onClose, locale }) => {
  const { scope, wallets, addTransaction } = useLedger();
  const isRw = locale === "rw";

  const [txType, setTxType] = useState<"income" | "expense">("expense");
  const [walletName, setWalletName] = useState(wallets[0]?.name || "");
  const [amount, setAmount] = useState("");
  const [category, setCategory] = useState("");
  const [description, setDescription] = useState("");
  const [actorName, setActorName] = useState("");
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const numAmount = parseFloat(amount);

    if (isNaN(numAmount) || numAmount <= 0) {
      setErrorMsg(t(locale, "errors.invalid_amount"));
      return;
    }

    const selectedWallet = wallets.find((w) => w.name === walletName);
    if (txType === "expense" && selectedWallet && selectedWallet.balance < numAmount) {
      setErrorMsg(t(locale, "errors.insufficient_custody", { wallet: walletName, available: selectedWallet.balance.toLocaleString(), requested: numAmount.toLocaleString() }));
      return;
    }

    addTransaction({
      txType,
      scope,
      walletName: walletName || wallets[0]?.name || "Cash",
      amount: numAmount,
      currency: "RWF",
      category: category.trim() || (isRw ? "Rusange" : "General"),
      description: description.trim() || (isRw ? "Nta bisobanuro" : "No note"),
      actorName: actorName.trim() || (scope === "personal" ? "Me" : "Family Member"),
    });

    onClose();
    // Reset form
    setAmount("");
    setCategory("");
    setDescription("");
    setErrorMsg(null);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
      <div className="bg-white dark:bg-brand-dark shadow-2xl border border-slate-200 dark:border-slate-800 max-w-md w-full overflow-hidden animate-in fade-in zoom-in-95 duration-150">
        <div className="p-5 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <h3 className="text-lg font-bold text-slate-900 dark:text-white">
            {t(locale, "transactions.log_new_transaction")}
          </h3>
          <button
            type="button"
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600 dark:hover:text-white font-bold p-1"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-4">
          {errorMsg && (
            <div className="p-3 bg-brand-crimson/10 border border-brand-crimson/30 text-brand-active-crimson text-xs font-semibold flex items-center">
              <AlertTriangle className="w-4 h-4 mr-2 shrink-0" />
              <span>{errorMsg}</span>
            </div>
          )}

          {/* Type selector */}
          <div className="grid grid-cols-2 gap-2 p-1 bg-slate-100 dark:bg-slate-800">
            <button
              type="button"
              onClick={() => {
                setTxType("expense");
                setErrorMsg(null);
              }}
              className={`py-2 text-xs font-bold transition flex items-center justify-center ${
                txType === "expense"
                  ? "bg-brand-crimson text-white shadow-sm"
                  : "text-slate-600 dark:text-slate-300"
              }`}
            >
              <ArrowDownRight className="w-3.5 h-3.5 mr-1" />
              <span>{t(locale, "transactions.expense")}</span>
            </button>
            <button
              type="button"
              onClick={() => {
                setTxType("income");
                setErrorMsg(null);
              }}
              className={`py-2 text-xs font-bold transition flex items-center justify-center ${
                txType === "income"
                  ? "bg-brand-steel-blue text-white shadow-sm"
                  : "text-slate-600 dark:text-slate-300"
              }`}
            >
              <ArrowUpRight className="w-3.5 h-3.5 mr-1" />
              <span>{t(locale, "transactions.income")}</span>
            </button>
          </div>

          <div>
            <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
              {t(locale, "wallets.held_by")}
            </label>
            <select
              value={walletName}
              onChange={(e) => {
                setWalletName(e.target.value);
                setErrorMsg(null);
              }}
              className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-steel-blue"
            >
              {wallets.map((w) => (
                <option key={w.id} value={w.name}>
                  {w.name} ({w.balance.toLocaleString()} {w.currency})
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
              {t(locale, "transactions.amount")}
            </label>
            <input
              type="number"
              placeholder="e.g. 25000"
              value={amount}
              onChange={(e) => {
                setAmount(e.target.value);
                setErrorMsg(null);
              }}
              required
              className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-steel-blue"
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
                {t(locale, "transactions.category")}
              </label>
              <input
                type="text"
                placeholder={isRw ? "Isoko, Umuriro..." : "Groceries, Utilities..."}
                value={category}
                onChange={(e) => setCategory(e.target.value)}
                className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-steel-blue"
              />
            </div>
            <div>
              <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
                {t(locale, "transactions.authorized_by")}
              </label>
              <input
                type="text"
                placeholder={isRw ? "Amazina yawe..." : "Your name..."}
                value={actorName}
                onChange={(e) => setActorName(e.target.value)}
                className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-steel-blue"
              />
            </div>
          </div>

          <div>
            <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
              {t(locale, "transactions.description")}
            </label>
            <textarea
              rows={2}
              placeholder={isRw ? "Andika impamvu..." : "Brief description..."}
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full px-3 py-2 text-sm border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-steel-blue resize-none"
            />
          </div>

          <div className="pt-2 flex items-center justify-end space-x-3">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 text-xs font-semibold text-slate-500 hover:text-slate-700 dark:hover:text-slate-200"
            >
              {t(locale, "common.cancel")}
            </button>
            <button
              type="submit"
              className="px-5 py-2 bg-brand-dark text-brand-cream hover:bg-brand-dark/90 dark:bg-brand-steel-blue dark:text-white font-bold text-xs shadow-md transition"
            >
              {t(locale, "common.save")}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
