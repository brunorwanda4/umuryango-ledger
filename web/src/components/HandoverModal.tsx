"use client";

import React, { useState } from "react";
import { Handshake, AlertTriangle, X } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";
import { t } from "@/lib/i18n";

interface HandoverModalProps {
  isOpen: boolean;
  onClose: () => void;
  locale: "rw" | "en";
}

export const HandoverModal: React.FC<HandoverModalProps> = ({ isOpen, onClose, locale }) => {
  const { wallets, addHandover } = useLedger();
  const isRw = locale === "rw";

  const [fromWallet, setFromWallet] = useState(wallets[0]?.name || "");
  const [toWallet, setToWallet] = useState(wallets[1]?.name || "");
  const [amount, setAmount] = useState("");
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

    if (fromWallet === toWallet) {
      setErrorMsg(t(locale, "errors.invalid_handover"));
      return;
    }

    const sourceWallet = wallets.find((w) => w.name === fromWallet);
    if (sourceWallet && sourceWallet.balance < numAmount) {
      setErrorMsg(t(locale, "errors.insufficient_custody", { wallet: fromWallet, available: sourceWallet.balance.toLocaleString(), requested: numAmount.toLocaleString() }));
      return;
    }

    addHandover(
      fromWallet,
      toWallet,
      numAmount,
      actorName.trim() || (isRw ? "Umubikizi" : "Custodian"),
    );

    onClose();
    setAmount("");
    setErrorMsg(null);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
      <div className="bg-white dark:bg-brand-dark rounded-2xl shadow-2xl border border-slate-200 dark:border-slate-800 max-w-md w-full overflow-hidden animate-in fade-in zoom-in-95 duration-150">
        <div className="p-5 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <Handshake className="w-5 h-5 text-brand-dark dark:text-brand-cream" />
            <h3 className="text-lg font-bold text-slate-900 dark:text-white">
              {t(locale, "transactions.handover_title")}
            </h3>
          </div>
          <button
            type="button"
            onClick={onClose}
            className="text-slate-400 hover:text-slate-600 dark:hover:text-white font-bold p-1 rounded-lg"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-4">
          {errorMsg && (
            <div className="p-3 rounded-xl bg-brand-crimson/10 border border-brand-crimson/30 text-brand-active-crimson text-xs font-semibold flex items-center">
              <AlertTriangle className="w-4 h-4 mr-2 shrink-0" />
              <span>{errorMsg}</span>
            </div>
          )}

          <div>
            <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
              {t(locale, "transactions.outgoing_wallet")}
            </label>
            <select
              value={fromWallet}
              onChange={(e) => {
                setFromWallet(e.target.value);
                setErrorMsg(null);
              }}
              className="w-full px-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-dark"
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
              {t(locale, "transactions.incoming_wallet")}
            </label>
            <select
              value={toWallet}
              onChange={(e) => {
                setToWallet(e.target.value);
                setErrorMsg(null);
              }}
              className="w-full px-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-dark"
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
              placeholder="e.g. 50000"
              value={amount}
              onChange={(e) => {
                setAmount(e.target.value);
                setErrorMsg(null);
              }}
              required
              className="w-full px-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-dark"
            />
          </div>

          <div>
            <label className="block text-xs font-semibold text-slate-600 dark:text-slate-300 mb-1">
              {t(locale, "transactions.authorized_by")}
            </label>
            <input
              type="text"
              placeholder={isRw ? "Amazina y'uwohereje..." : "Name of initiator..."}
              value={actorName}
              onChange={(e) => setActorName(e.target.value)}
              className="w-full px-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-brand-dark"
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
              className="px-5 py-2 rounded-xl bg-brand-dark text-brand-cream hover:bg-brand-dark/90 dark:bg-brand-steel-blue dark:text-white font-bold text-xs shadow-md transition"
            >
              {t(locale, "common.confirm")}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
