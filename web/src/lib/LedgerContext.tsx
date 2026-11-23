"use client";

import React, { createContext, useContext, useState, useEffect, useCallback } from "react";
import {
  User,
  Family,
  PendingFamilyInvite,
  RegisterPayload,
  loginUser,
  registerUser,
  createFamily as apiCreateFamily,
  joinFamily as apiJoinFamily,
  getPendingInvites as apiGetPendingInvites,
} from "@/lib/api";

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
  currentUser: User | null;
  token: string | null;
  pendingInvites: PendingFamilyInvite[];
  login: (phone: string, pass: string) => Promise<void>;
  register: (payload: RegisterPayload) => Promise<void>;
  createFamily: (
    name: string,
    memberPhones: string[],
  ) => Promise<{ family: Family; invite_code: string }>;
  joinFamily: (inviteCodeOrId: string, isCode?: boolean) => Promise<void>;
  skipOnboarding: () => void;
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
    balance: 2100000,
    custodianName: "Me",
    currency: "RWF",
  },
  {
    id: "w-p-momo",
    name: "Personal MTN MoMo",
    type: "momo",
    balance: 380000,
    custodianName: "Me",
    currency: "RWF",
  },
  {
    id: "w-p-cash",
    name: "Cash in Pocket",
    type: "cash",
    balance: 50000,
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
    amount: 150000,
    currency: "RWF",
    category: "Umusanzu w'Ubuhinzi",
    description: "Umusaruro w'ibigori (Musanze)",
    actorName: "Papa",
    occurredAt: "2026-11-23 09:30",
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
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [pendingInvites, setPendingInvites] = useState<PendingFamilyInvite[]>([]);

  // Initialize token and user from localStorage on client
  useEffect(() => {
    if (typeof window !== "undefined") {
      const storedToken = localStorage.getItem("umuryango_token");
      const storedUser = localStorage.getItem("umuryango_user");
      if (storedToken) setToken(storedToken);
      if (storedUser) {
        try {
          setCurrentUser(JSON.parse(storedUser));
        } catch {
          // Ignore invalid parse
        }
      }
    }
  }, []);

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

  const login = useCallback(
    async (phone: string, pass: string) => {
      const res = await loginUser({ phone, password: pass }, currentLocale);
      setToken(res.token);
      setCurrentUser(res.user);
      setPendingInvites(res.pending_invites || []);
      if (typeof window !== "undefined") {
        localStorage.setItem("umuryango_token", res.token);
        localStorage.setItem("umuryango_user", JSON.stringify(res.user));
      }
      addToast({
        type: "info",
        title: currentLocale === "rw" ? "Mwinjiye neza" : "Logged in successfully",
        message:
          currentLocale === "rw"
            ? `Murakaza neza, ${res.user.full_name}`
            : `Welcome back, ${res.user.full_name}`,
      });
    },
    [currentLocale, addToast],
  );

  const register = useCallback(
    async (payload: RegisterPayload) => {
      const res = await registerUser(payload, currentLocale);
      setToken(res.token);
      setCurrentUser(res.user);
      setPendingInvites(res.pending_invites || []);
      if (typeof window !== "undefined") {
        localStorage.setItem("umuryango_token", res.token);
        localStorage.setItem("umuryango_user", JSON.stringify(res.user));
      }
      addToast({
        type: "info",
        title: currentLocale === "rw" ? "Konti yafunguwe" : "Account Created",
        message:
          currentLocale === "rw"
            ? "Konti yanyu yaremwe neza!"
            : "Your account was created successfully!",
      });
    },
    [currentLocale, addToast],
  );

  const createFamily = useCallback(
    async (name: string, memberPhones: string[]) => {
      if (!token) throw new Error("Authentication required");
      const res = await apiCreateFamily(
        { name, member_phones: memberPhones },
        token,
        currentLocale,
      );
      setScope("family");
      addToast({
        type: "info",
        title: currentLocale === "rw" ? "Umuryango waremwe" : "Family Created",
        message:
          currentLocale === "rw"
            ? `Umuryango '${name}' waremwe! Kode y'ubutumire: ${res.invite_code}`
            : `Family '${name}' created! Invite Code: ${res.invite_code}`,
      });
      return res;
    },
    [token, currentLocale, addToast],
  );

  const joinFamily = useCallback(
    async (inviteCodeOrId: string, isCode = true) => {
      if (!token) throw new Error("Authentication required");
      const payload = isCode ? { invite_code: inviteCodeOrId } : { invite_id: inviteCodeOrId };
      const res = await apiJoinFamily(payload, token, currentLocale);
      setScope("family");
      setPendingInvites((prev) => prev.filter((inv) => inv.id !== inviteCodeOrId));
      addToast({
        type: "info",
        title: currentLocale === "rw" ? "Mwinjiye mu muryango" : "Joined Family",
        message:
          currentLocale === "rw"
            ? `Mwinjiye muri ${res.family.name} neza!`
            : `Successfully joined ${res.family.name}!`,
      });
    },
    [token, currentLocale, addToast],
  );

  const skipOnboarding = useCallback(() => {
    setScope("personal");
    addToast({
      type: "info",
      title: currentLocale === "rw" ? "Konti Bwite" : "Personal Scope",
      message:
        currentLocale === "rw"
          ? "Ubu murimo gukoresha Amafaranga Yanjye (Konti Bwite)."
          : "You are currently using your Personal wallet ledger.",
    });
  }, [currentLocale, addToast]);

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
              : "New Income Logged"
            : isRw
              ? "Amafaranga Yakoreshejwe"
              : "Expense Logged",
        message: `${tx.actorName} ${
          tx.txType === "income" ? (isRw ? "yongeye" : "added") : isRw ? "yakoresheje" : "spent"
        } ${tx.amount.toLocaleString()} ${tx.currency} (${tx.category})`,
      });
    },
    [currentLocale, addToast],
  );

  const addHandover = useCallback(
    (fromWallet: string, toWallet: string, amount: number, actor: string) => {
      const newTx: LedgerTransaction = {
        id: "tx-" + Date.now(),
        txType: "handover",
        scope: "family",
        walletName: fromWallet,
        toWalletName: toWallet,
        amount,
        currency: "RWF",
        category: "Ihererekanya ry'Ubwiringiro",
        description: `Handover from ${fromWallet} to ${toWallet}`,
        actorName: actor,
        occurredAt: new Date().toISOString().replace("T", " ").substring(0, 16),
      };

      setTransactions((prev) => [newTx, ...prev]);

      // Update wallets: deduct source, credit destination
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
        title: isRw ? "Ubwiringiro Bwahererekanyijwe" : "Custody Handover Completed",
        message: isRw
          ? `${actor} yahererekanyije ${amount.toLocaleString()} RWF kuva kuri '${fromWallet}' ajya kuri '${toWallet}'.`
          : `${actor} handed over ${amount.toLocaleString()} RWF from '${fromWallet}' to '${toWallet}'.`,
      });
    },
    [currentLocale, addToast],
  );

  // WebSocket Live Updates
  useEffect(() => {
    const wsUrl = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:4433/ws";
    let ws: WebSocket | null = null;
    let reconnectTimeout: ReturnType<typeof setTimeout>;

    const connect = () => {
      try {
        ws = new WebSocket(wsUrl);

        ws.onmessage = (event) => {
          try {
            const data = JSON.parse(event.data);
            if (data.type === "IncomeAdded") {
              const p = data.payload;
              const isRw = currentLocale === "rw";
              addToast({
                type: "income",
                title: isRw ? "Amafaranga Mashya Yakiriwe" : "New Income Logged",
                message: isRw
                  ? `${p.added_by_name} yongeye ${p.amount.toLocaleString()} ${p.currency} kuri konti '${p.wallet_name}' (${p.source_description}).`
                  : `${p.added_by_name} added ${p.amount.toLocaleString()} ${p.currency} to '${p.wallet_name}' (${p.source_description}).`,
              });
            } else if (data.type === "ExpenseLogged") {
              const p = data.payload;
              const isRw = currentLocale === "rw";
              addToast({
                type: "expense",
                title: isRw ? "Amafaranga Yakoreshejwe" : "Expense Logged",
                message: isRw
                  ? `${p.logged_by_name} yakoresheje ${p.amount.toLocaleString()} ${p.currency} kuri konti '${p.wallet_name}' (${p.category}).`
                  : `${p.logged_by_name} logged ${p.amount.toLocaleString()} ${p.currency} from '${p.wallet_name}' (${p.category}).`,
              });
            } else if (data.type === "HandoverCompleted") {
              const p = data.payload;
              const isRw = currentLocale === "rw";
              addToast({
                type: "handover",
                title: isRw ? "Ubwiringiro Bwahererekanyijwe" : "Custody Handover Completed",
                message: isRw
                  ? `${p.initiated_by_name} yahererekanyije ${p.amount.toLocaleString()} ${p.currency} kuva kuri '${p.from_wallet_name}' ajya kuri '${p.to_wallet_name}'.`
                  : `${p.initiated_by_name} handed over ${p.amount.toLocaleString()} ${p.currency} from '${p.from_wallet_name}' to '${p.to_wallet_name}'.`,
              });
            }
          } catch {
            // Non-JSON WS frame
          }
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
        currentUser,
        token,
        pendingInvites,
        login,
        register,
        createFamily,
        joinFamily,
        skipOnboarding,
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
