"use client";

import React, { createContext, useContext, useState, useEffect, useCallback } from "react";

export type ScopeType = "family" | "personal";
export type ThemeType = "light" | "dark";

export interface CustodyWallet {
  id: string;
  name: string;
  type: "bank" | "momo" | "cash";
  balance: number;
  custodianName: string;
  currency: string;
}

export interface LedgerTransaction {
  id: string;
  txType: "income" | "expense" | "handover";
  scope: ScopeType;
  walletName: string;
  amount: number;
  currency: string;
  category: string;
  description: string;
  actorName: string;
  occurredAt: string;
  toWalletName?: string;
}

export interface ToastMessage {
  id: string;
  type: "income" | "expense" | "handover" | "project" | "info";
  title: string;
  message: string;
  timestamp: string;
}

interface LedgerContextType {
  scope: ScopeType;
  setScope: (scope: ScopeType) => void;
  theme: ThemeType;
  toggleTheme: () => void;
  wallets: CustodyWallet[];
  transactions: LedgerTransaction[];
  toasts: ToastMessage[];
  dismissToast: (id: string) => void;
  addTransaction: (tx: Omit<LedgerTransaction, "id" | "occurredAt">) => void;
  addHandover: (fromWallet: string, toWallet: string, amount: number, actor: string) => void;
  kpis: {
    totalAssets: number;
    cashInHand: number;
    momoTotal: number;
    bankTotal: number;
    outstandingDebts: number;
  };
}

const LedgerContext = createContext<LedgerContextType | undefined>(undefined);

const INITIAL_FAMILY_WALLETS: CustodyWallet[] = [
  {
    id: "w-bk",
    name: "BK Family Account",
    type: "bank",
    balance: 8200000,
    custodianName: "Papa",
    currency: "RWF",
  },
  {
    id: "w-momo",
    name: "MTN MoMo Treasury",
    type: "momo",
    balance: 1450000,
    custodianName: "Mama",
    currency: "RWF",
  },
  {
    id: "w-cash-mama",
    name: "Cash in Hand (Mama)",
    type: "cash",
    balance: 200000,
    custodianName: "Mama",
    currency: "RWF",
  },
  {
    id: "w-cash-papa",
    name: "Cash in Hand (Papa)",
    type: "cash",
    balance: 150000,
    custodianName: "Papa",
    currency: "RWF",
  },
];

const INITIAL_PERSONAL_WALLETS: CustodyWallet[] = [
  {
    id: "w-p-bk",
    name: "Equity Bank Personal",
    type: "bank",
    balance: 1250000,
    custodianName: "Me",
    currency: "RWF",
  },
  {
    id: "w-p-momo",
    name: "Personal MTN MoMo",
    type: "momo",
    balance: 185000,
    custodianName: "Me",
    currency: "RWF",
  },
  {
    id: "w-p-cash",
    name: "Pocket Cash",
    type: "cash",
    balance: 35000,
    custodianName: "Me",
    currency: "RWF",
  },
];

const INITIAL_TRANSACTIONS: LedgerTransaction[] = [
  {
    id: "tx-1",
    txType: "income",
    scope: "family",
    walletName: "MTN MoMo Treasury",
    amount: 500000,
    currency: "RWF",
    category: "Umuganda w'Umuryango",
    description: "Umusanzu w'ukwezi kwa 11 (Kagabo)",
    actorName: "Kagabo Jean",
    occurredAt: "2026-11-22 14:30",
  },
  {
    id: "tx-2",
    txType: "expense",
    scope: "family",
    walletName: "Cash in Hand (Mama)",
    amount: 45000,
    currency: "RWF",
    category: "Amasoko y'Ibiribwa",
    description: "Kugura umuceri n'amavuta ku isoko",
    actorName: "Mama",
    occurredAt: "2026-11-22 10:15",
  },
  {
    id: "tx-3",
    txType: "handover",
    scope: "family",
    walletName: "MTN MoMo Treasury",
    toWalletName: "Cash in Hand (Papa)",
    amount: 100000,
    currency: "RWF",
    category: "Ihererekanya",
    description: "Kwishyura abafundi basana urugo",
    actorName: "Mama",
    occurredAt: "2026-11-21 16:45",
  },
  {
    id: "tx-4",
    txType: "expense",
    scope: "personal",
    walletName: "Personal MTN MoMo",
    amount: 15000,
    currency: "RWF",
    category: "Transport",
    description: "Fuel & Taxi fare",
    actorName: "Me",
    occurredAt: "2026-11-22 18:00",
  },
];

export const LedgerProvider: React.FC<{
  children: React.ReactNode;
  currentLocale: "rw" | "en";
}> = ({ children, currentLocale }) => {
  const [scope, setScope] = useState<ScopeType>("family");
  const [theme, setTheme] = useState<ThemeType>("light");
  const [wallets, setWallets] = useState<CustodyWallet[]>(INITIAL_FAMILY_WALLETS);
  const [transactions, setTransactions] = useState<LedgerTransaction[]>(INITIAL_TRANSACTIONS);
  const [toasts, setToasts] = useState<ToastMessage[]>([]);

  // Update theme class on HTML element
  useEffect(() => {
    const root = document.documentElement;
    if (theme === "dark") {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }, [theme]);

  const toggleTheme = useCallback(() => {
    setTheme((prev) => (prev === "light" ? "dark" : "light"));
  }, []);

  // Sync active wallets based on Scope
  useEffect(() => {
    if (scope === "family") {
      setWallets(INITIAL_FAMILY_WALLETS);
    } else {
      setWallets(INITIAL_PERSONAL_WALLETS);
    }
  }, [scope]);

  const dismissToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const addToast = useCallback((toast: Omit<ToastMessage, "id" | "timestamp">) => {
    const newToast: ToastMessage = {
      ...toast,
      id: "toast-" + Math.random().toString(36).substring(2, 9),
      timestamp: new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
    };
    setToasts((prev) => [newToast, ...prev.slice(0, 4)]);
  }, []);

  const addTransaction = useCallback(
    (tx: Omit<LedgerTransaction, "id" | "occurredAt">) => {
      const newTx: LedgerTransaction = {
        ...tx,
        id: "tx-" + Date.now(),
        occurredAt: new Date().toISOString().replace("T", " ").substring(0, 16),
      };

      setTransactions((prev) => [newTx, ...prev]);

      // Update wallet balance optimistically
      setWallets((prev) =>
        prev.map((w) => {
          if (w.name === tx.walletName) {
            const nextBal =
              tx.txType === "income" ? w.balance + tx.amount : Math.max(0, w.balance - tx.amount);
            return { ...w, balance: nextBal };
          }
          return w;
        }),
      );

      // Toast notification
      const isRw = currentLocale === "rw";
      addToast({
        type: tx.txType,
        title:
          tx.txType === "income"
            ? isRw
              ? "Amafaranga Mashya Yakiriwe"
              : "New Income Received"
            : isRw
              ? "Amafaranga Yakoreshejwe"
              : "Expense Recorded",
        message: `${tx.amount.toLocaleString()} ${tx.currency} - ${tx.walletName} (${tx.actorName})`,
      });
    },
    [currentLocale, addToast],
  );

  const addHandover = useCallback(
    (fromWallet: string, toWallet: string, amount: number, actor: string) => {
      const newTx: LedgerTransaction = {
        id: "tx-ho-" + Date.now(),
        txType: "handover",
        scope,
        walletName: fromWallet,
        toWalletName: toWallet,
        amount,
        currency: "RWF",
        category: "Ihererekanya ry'Umutungo",
        description: `Handover from ${fromWallet} to ${toWallet}`,
        actorName: actor,
        occurredAt: new Date().toISOString().replace("T", " ").substring(0, 16),
      };

      setTransactions((prev) => [newTx, ...prev]);

      setWallets((prev) =>
        prev.map((w) => {
          if (w.name === fromWallet) {
            return { ...w, balance: Math.max(0, w.balance - amount) };
          }
          if (w.name === toWallet) {
            return { ...w, balance: w.balance + amount };
          }
          return w;
        }),
      );

      const isRw = currentLocale === "rw";
      addToast({
        type: "handover",
        title: isRw ? "Ihererekanya ry'Umutungo Ryemejwe" : "Custody Handover Completed",
        message: `${amount.toLocaleString()} RWF: ${fromWallet} → ${toWallet}`,
      });
    },
    [scope, currentLocale, addToast],
  );

  // WebSocket Live Subscription to Actix Backend
  useEffect(() => {
    let ws: WebSocket | null = null;
    let reconnectTimeout: any = null;

    const connect = () => {
      try {
        const wsUrl =
          (process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8080/api/v1/ws") +
          "?token=demo-live-token";
        ws = new WebSocket(wsUrl);

        ws.onopen = () => {
          // Connected cleanly
        };

        ws.onmessage = (event) => {
          try {
            const data = JSON.parse(event.data);
            const isRw = currentLocale === "rw";

            if (data.type === "IncomeAdded") {
              addToast({
                type: "income",
                title: isRw ? "Amafaranga Mashya (WebSocket)" : "New Inflow (Live)",
                message: `${data.amount?.toLocaleString()} ${data.currency || "RWF"} yinjijwe kuri ${data.wallet_name} na ${data.added_by_name}`,
              });
            } else if (data.type === "ExpenseLogged") {
              addToast({
                type: "expense",
                title: isRw ? "Igikorwa Gishya cy'Isoko" : "New Expense Logged (Live)",
                message: `${data.amount?.toLocaleString()} ${data.currency || "RWF"} kuri ${data.wallet_name} (${data.category})`,
              });
            } else if (data.type === "HandoverCompleted") {
              addToast({
                type: "handover",
                title: isRw ? "Ihererekanya Ryemejwe" : "Handover Confirmed (Live)",
                message: `${data.amount?.toLocaleString()} RWF kuva kuri ${data.from_wallet_name} yageze kuri ${data.to_wallet_name}`,
              });
            }
          } catch {
            // Ignore parse errors on ping/pong frames
          }
        };

        ws.onerror = () => {
          // Backend may be offline in dev
        };

        ws.onclose = () => {
          reconnectTimeout = setTimeout(connect, 10000);
        };
      } catch {
        // Fallback for SSR or non-browser environments
      }
    };

    connect();

    return () => {
      if (ws) ws.close();
      if (reconnectTimeout) clearTimeout(reconnectTimeout);
    };
  }, [currentLocale, addToast]);

  // Compute KPIs
  const totalAssets = wallets.reduce((acc, w) => acc + w.balance, 0);
  const cashInHand = wallets
    .filter((w) => w.type === "cash")
    .reduce((acc, w) => acc + w.balance, 0);
  const momoTotal = wallets.filter((w) => w.type === "momo").reduce((acc, w) => acc + w.balance, 0);
  const bankTotal = wallets.filter((w) => w.type === "bank").reduce((acc, w) => acc + w.balance, 0);

  return (
    <LedgerContext.Provider
      value={{
        scope,
        setScope,
        theme,
        toggleTheme,
        wallets,
        transactions,
        toasts,
        dismissToast,
        addTransaction,
        addHandover,
        kpis: {
          totalAssets,
          cashInHand,
          momoTotal,
          bankTotal,
          outstandingDebts: scope === "family" ? 150000 : 0,
        },
      }}
    >
      {children}
    </LedgerContext.Provider>
  );
};

export const useLedger = () => {
  const context = useContext(LedgerContext);
  if (!context) {
    throw new Error("useLedger must be used within a LedgerProvider");
  }
  return context;
};
