"use client";

import React, { useState } from "react";
import Link from "next/link";
import { useRouter, useParams } from "next/navigation";
import { User as UserIcon, Phone, Lock, ArrowRight, UserPlus, AlertCircle } from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";

export default function RegisterPage() {
  const router = useRouter();
  const params = useParams();
  const locale = (params?.locale as "rw" | "en") || "rw";
  const isRw = locale === "rw";

  const { register } = useLedger();
  const [fullName, setFullName] = useState("");
  const [phone, setPhone] = useState("");
  const [gender, setGender] = useState("MALE");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (password !== confirmPassword) {
      setError(
        isRw
          ? "Amagambo banga ntabwo ahura. Ongera uyandike neza."
          : "Passwords do not match. Please enter the same password twice.",
      );
      return;
    }

    try {
      setLoading(true);
      await register({
        full_name: fullName.trim(),
        phone: phone.trim(),
        gender,
        password,
        confirm_password: confirmPassword,
        preferred_locale: locale,
      });
      router.push(`/${locale}/onboarding`);
    } catch (err: unknown) {
      const msg =
        err instanceof Error
          ? err.message
          : isRw
            ? "Ikosa mu kurema konti."
            : "Registration failed.";
      setError(msg);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-[75vh] items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8 bg-white dark:bg-slate-900 p-8 rounded-2xl shadow-xl border border-slate-200 dark:border-slate-800 transition-colors">
        <div className="text-center space-y-2">
          <div className="inline-flex p-3 rounded-2xl bg-brand-crimson/10 text-brand-active-crimson dark:bg-brand-crimson/20">
            <UserPlus className="w-8 h-8" />
          </div>
          <h2 className="text-2xl font-black text-brand-dark dark:text-brand-cream tracking-tight">
            {isRw ? "Fungura Konti Nshya" : "Create your Account"}
          </h2>
          <p className="text-xs text-slate-500 dark:text-slate-400">
            {isRw
              ? "Uburyo bwizewe bwo gukurikirana umutungo w'umuryango."
              : "Dual-scope personal and family accountability in Rwanda and Diaspora."}
          </p>
        </div>

        {error && (
          <div className="p-3 rounded-xl bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-800 flex items-center space-x-2 text-xs font-semibold text-red-600 dark:text-red-400">
            <AlertCircle className="w-4 h-4 shrink-0" />
            <span>{error}</span>
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-1">
            <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
              {isRw ? "Amazina Yose" : "Full Name"}
            </label>
            <div className="relative">
              <UserIcon className="w-4 h-4 text-slate-400 absolute left-3 top-3" />
              <input
                type="text"
                value={fullName}
                onChange={(e) => setFullName(e.target.value)}
                placeholder="Kwizera Aimable"
                className="w-full pl-9 pr-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                required
              />
            </div>
          </div>

          <div className="space-y-1">
            <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
              {isRw ? "Nimero ya Telefoni" : "Phone Number"}
            </label>
            <div className="relative">
              <Phone className="w-4 h-4 text-slate-400 absolute left-3 top-3" />
              <input
                type="tel"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
                placeholder="+250 788 123 456"
                className="w-full pl-9 pr-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                required
              />
            </div>
          </div>

          <div className="space-y-1">
            <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
              {isRw ? "Igitsina" : "Gender"}
            </label>
            <div className="grid grid-cols-2 gap-3">
              <button
                type="button"
                onClick={() => setGender("MALE")}
                className={`py-2 px-3 text-xs font-bold rounded-xl border transition cursor-pointer ${
                  gender === "MALE"
                    ? "bg-brand-crimson text-white border-brand-crimson"
                    : "bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border-slate-300 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-700"
                }`}
              >
                {isRw ? "Gabo (Male)" : "Male"}
              </button>
              <button
                type="button"
                onClick={() => setGender("FEMALE")}
                className={`py-2 px-3 text-xs font-bold rounded-xl border transition cursor-pointer ${
                  gender === "FEMALE"
                    ? "bg-brand-crimson text-white border-brand-crimson"
                    : "bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 border-slate-300 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-700"
                }`}
              >
                {isRw ? "Gore (Female)" : "Female"}
              </button>
            </div>
          </div>

          <div className="space-y-1">
            <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
              {isRw ? "Ijambo banga" : "Password"}
            </label>
            <div className="relative">
              <Lock className="w-4 h-4 text-slate-400 absolute left-3 top-3" />
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="••••••••"
                className="w-full pl-9 pr-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                required
              />
            </div>
          </div>

          <div className="space-y-1">
            <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
              {isRw ? "Ongera wandike Ijambo banga" : "Confirm Password"}
            </label>
            <div className="relative">
              <Lock className="w-4 h-4 text-slate-400 absolute left-3 top-3" />
              <input
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="••••••••"
                className="w-full pl-9 pr-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                required
              />
            </div>
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full py-2.5 px-4 rounded-xl font-bold text-sm text-white bg-brand-crimson hover:bg-brand-active-crimson active:scale-[0.99] transition shadow-md flex items-center justify-center space-x-2 disabled:opacity-50 cursor-pointer"
          >
            <span>
              {loading
                ? isRw
                  ? "Turimo kurema konti..."
                  : "Creating account..."
                : isRw
                  ? "Fungura Konti"
                  : "Create Account"}
            </span>
            {!loading && <ArrowRight className="w-4 h-4" />}
          </button>
        </form>

        <div className="text-center pt-2 border-t border-slate-100 dark:border-slate-800">
          <p className="text-xs text-slate-600 dark:text-slate-400">
            {isRw ? "Usanzwe ufite konti? " : "Already have an account? "}
            <Link
              href={`/${locale}/login`}
              className="font-bold text-brand-active-crimson hover:underline"
            >
              {isRw ? "Injira hano" : "Sign In"}
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
}
