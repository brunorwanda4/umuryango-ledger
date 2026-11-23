"use client";

import React, { useState } from "react";
import { useRouter, useParams } from "next/navigation";
import {
  Users,
  KeyRound,
  Plus,
  ArrowRight,
  Sparkles,
  CheckCircle,
  AlertCircle,
  UserCheck,
} from "lucide-react";
import { useLedger } from "@/lib/LedgerContext";

export default function OnboardingPage() {
  const router = useRouter();
  const params = useParams();
  const locale = (params?.locale as "rw" | "en") || "rw";
  const isRw = locale === "rw";

  const { pendingInvites, createFamily, joinFamily, skipOnboarding } = useLedger();

  // Option 1: Create Family State
  const [familyName, setFamilyName] = useState("");
  const [memberPhones, setMemberPhones] = useState<string[]>([""]);
  const [creating, setCreating] = useState(false);
  const [createError, setCreateError] = useState<string | null>(null);

  // Option 2: Join Family State
  const [inviteCode, setInviteCode] = useState("");
  const [joining, setJoining] = useState(false);
  const [joinError, setJoinError] = useState<string | null>(null);

  // Add Phone row
  const addPhoneRow = () => {
    setMemberPhones((prev) => [...prev, ""]);
  };

  const updatePhoneRow = (index: number, val: string) => {
    setMemberPhones((prev) => {
      const copy = [...prev];
      copy[index] = val;
      return copy;
    });
  };

  // Handle Create Family
  const handleCreateSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setCreateError(null);
    if (!familyName.trim()) {
      setCreateError(isRw ? "Izina ry'umuryango rirakenewe." : "Family name is required.");
      return;
    }

    try {
      setCreating(true);
      const filteredPhones = memberPhones.map((p) => p.trim()).filter((p) => p.length > 0);
      await createFamily(familyName.trim(), filteredPhones);
      router.push(`/${locale}/dashboard`);
    } catch (err: unknown) {
      const msg =
        err instanceof Error
          ? err.message
          : isRw
            ? "Ikosa mu kurema umuryango."
            : "Failed to create family.";
      setCreateError(msg);
    } finally {
      setCreating(false);
    }
  };

  // Handle Join Family by Code
  const handleJoinSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setJoinError(null);
    const cleanCode = inviteCode.trim().toUpperCase();
    if (cleanCode.length !== 4) {
      setJoinError(
        isRw
          ? "Kode y'umuryango igomba kuba ifite inyuguti 4."
          : "Family invite code must be 4 characters.",
      );
      return;
    }

    try {
      setJoining(true);
      await joinFamily(cleanCode, true);
      router.push(`/${locale}/dashboard`);
    } catch (err: unknown) {
      const msg =
        err instanceof Error
          ? err.message
          : isRw
            ? "Kode y'umuryango y'inyuguti 4 ntabwo ari yo. Ongera ugerageze."
            : "Invalid 4-character family code. Please check and try again.";
      setJoinError(msg);
    } finally {
      setJoining(false);
    }
  };

  // Handle 1-click Join from Pending Invite
  const handleAcceptPending = async (inviteId: string) => {
    try {
      await joinFamily(inviteId, false);
      router.push(`/${locale}/dashboard`);
    } catch (err: unknown) {
      const msg =
        err instanceof Error
          ? err.message
          : isRw
            ? "Ikosa mu kwemera ubutumire."
            : "Failed to join family.";
      setJoinError(msg);
    }
  };

  // Handle Skip to Personal Scope
  const handleSkip = () => {
    skipOnboarding();
    router.push(`/${locale}/dashboard`);
  };

  return (
    <div className="max-w-3xl mx-auto py-10 px-4 space-y-8">
      {/* Header */}
      <div className="text-center space-y-3">
        <div className="inline-flex p-3 rounded-2xl bg-brand-crimson/10 text-brand-active-crimson dark:bg-brand-crimson/20">
          <Sparkles className="w-8 h-8" />
        </div>
        <h1 className="text-3xl font-black text-brand-dark dark:text-brand-cream tracking-tight">
          {isRw ? "Guhuza Umuryango wawe" : "Connect with your Family"}
        </h1>
        <p className="text-sm text-slate-600 dark:text-slate-400 max-w-lg mx-auto">
          {isRw
            ? "Hitamo uko utangira: Rema umuryango mushya, winjire mu muryango uhereye kuri kode y'inyuguti 4, cyangwa ukoreshe konti yawe bwite."
            : "Choose how to proceed: create a family ledger, join using a 4-character code, or start with your personal ledger."}
        </p>
      </div>

      {/* Pending Invites Banner (Shown automatically if invites exist) */}
      {pendingInvites.length > 0 && (
        <div className="bg-amber-50 dark:bg-amber-950/40 border border-amber-300 dark:border-amber-800 rounded-2xl p-6 space-y-4 shadow-sm">
          <div className="flex items-center space-x-2 text-amber-900 dark:text-amber-200 font-bold text-sm">
            <UserCheck className="w-5 h-5 text-amber-600 dark:text-amber-400" />
            <span>
              {isRw ? "Ubutumire bw'Umuryango bugutegereje" : "Pending Family Invitations"}
            </span>
          </div>
          <div className="space-y-3">
            {pendingInvites.map((invite) => (
              <div
                key={invite.id}
                className="bg-white dark:bg-slate-900 p-4 rounded-xl border border-amber-200 dark:border-amber-900/60 flex items-center justify-between"
              >
                <div>
                  <h4 className="font-bold text-sm text-slate-900 dark:text-slate-100">
                    {isRw ? "Wongewe muri " : "You were added to "}
                    <span className="text-brand-active-crimson">{invite.family_name}</span>
                  </h4>
                  <p className="text-xs text-slate-500 dark:text-slate-400">
                    {isRw
                      ? `Batumwe na: ${invite.invited_by_name}`
                      : `Invited by: ${invite.invited_by_name}`}
                  </p>
                </div>
                <button
                  type="button"
                  onClick={() => handleAcceptPending(invite.id)}
                  className="px-4 py-2 rounded-xl text-xs font-bold text-white bg-emerald-600 hover:bg-emerald-700 transition shadow cursor-pointer flex items-center space-x-1.5"
                >
                  <CheckCircle className="w-4 h-4" />
                  <span>{isRw ? "Injira ubu" : "Join Now"}</span>
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Options Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Option 1: Create Family */}
        <div className="bg-white dark:bg-slate-900 p-6 rounded-2xl shadow-md border border-slate-200 dark:border-slate-800 space-y-5 flex flex-col justify-between">
          <div className="space-y-4">
            <div className="flex items-center space-x-3">
              <div className="p-2.5 rounded-xl bg-brand-crimson/10 text-brand-active-crimson dark:bg-brand-crimson/20">
                <Users className="w-5 h-5" />
              </div>
              <h3 className="font-bold text-base text-brand-dark dark:text-brand-cream">
                {isRw ? "Ihitamo 1: Rema Umuryango" : "Option 1: Create Family"}
              </h3>
            </div>
            <p className="text-xs text-slate-500 dark:text-slate-400">
              {isRw
                ? "Shyiraho umuryango mushya maze utumire abandi banyamuryango ukoresheje telefone zabo."
                : "Establish a new shared custody ledger and invite family members by their phone number."}
            </p>

            {createError && (
              <div className="p-2.5 rounded-xl bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-800 flex items-center space-x-2 text-xs text-red-600 dark:text-red-400">
                <AlertCircle className="w-4 h-4 shrink-0" />
                <span>{createError}</span>
              </div>
            )}

            <form onSubmit={handleCreateSubmit} id="create-family-form" className="space-y-3">
              <div className="space-y-1">
                <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
                  {isRw ? "Izina ry'Umuryango" : "Family Name"}
                </label>
                <input
                  type="text"
                  value={familyName}
                  onChange={(e) => setFamilyName(e.target.value)}
                  placeholder={isRw ? "Umuryango w'Abahire" : "The Abahire Family"}
                  className="w-full px-3 py-2 text-sm rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                  required
                />
              </div>

              <div className="space-y-1">
                <div className="flex items-center justify-between">
                  <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
                    {isRw ? "Ongeraho Telefone z'Abanyamuryango" : "Add Member Phone Numbers"}
                  </label>
                  <button
                    type="button"
                    onClick={addPhoneRow}
                    className="text-xs font-bold text-brand-active-crimson hover:underline flex items-center space-x-1 cursor-pointer"
                  >
                    <Plus className="w-3.5 h-3.5" />
                    <span>{isRw ? "Ongeraho" : "+ Add"}</span>
                  </button>
                </div>
                <div className="space-y-2">
                  {memberPhones.map((phone, idx) => (
                    <input
                      key={idx}
                      type="tel"
                      value={phone}
                      onChange={(e) => updatePhoneRow(idx, e.target.value)}
                      placeholder="+250 788 000 000"
                      className="w-full px-3 py-1.5 text-xs rounded-lg border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                    />
                  ))}
                </div>
              </div>
            </form>
          </div>

          <button
            form="create-family-form"
            type="submit"
            disabled={creating}
            className="w-full py-2.5 px-4 rounded-xl font-bold text-sm text-white bg-brand-crimson hover:bg-brand-active-crimson transition shadow-md flex items-center justify-center space-x-2 disabled:opacity-50 cursor-pointer"
          >
            <span>
              {creating
                ? isRw
                  ? "Turimo kurema..."
                  : "Creating..."
                : isRw
                  ? "Komeza kuri Dashboard"
                  : "Create & Continue"}
            </span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>

        {/* Option 2: Join Family */}
        <div className="bg-white dark:bg-slate-900 p-6 rounded-2xl shadow-md border border-slate-200 dark:border-slate-800 space-y-5 flex flex-col justify-between">
          <div className="space-y-4">
            <div className="flex items-center space-x-3">
              <div className="p-2.5 rounded-xl bg-brand-steel/10 text-brand-steel dark:bg-brand-steel/20">
                <KeyRound className="w-5 h-5" />
              </div>
              <h3 className="font-bold text-base text-brand-dark dark:text-brand-cream">
                {isRw ? "Ihitamo 2: Injira muri Kode" : "Option 2: Join with Code"}
              </h3>
            </div>
            <p className="text-xs text-slate-500 dark:text-slate-400">
              {isRw
                ? "Andika kode y'umuryango y'inyuguti 4 (urugero: HK0N) wagiriwe n'umukuru w'umuryango."
                : "Enter the 4-character family invite code (e.g. HK0N) shared by your family head."}
            </p>

            {joinError && (
              <div className="p-2.5 rounded-xl bg-red-50 dark:bg-red-950/40 border border-red-200 dark:border-red-800 flex items-center space-x-2 text-xs text-red-600 dark:text-red-400">
                <AlertCircle className="w-4 h-4 shrink-0" />
                <span>{joinError}</span>
              </div>
            )}

            <form onSubmit={handleJoinSubmit} id="join-family-form" className="space-y-3">
              <div className="space-y-1">
                <label className="text-xs font-bold text-slate-700 dark:text-slate-300">
                  {isRw ? "Kode y'Inyuguti 4 (Invite Code)" : "4-Character Family Code"}
                </label>
                <input
                  type="text"
                  maxLength={4}
                  value={inviteCode}
                  onChange={(e) => setInviteCode(e.target.value.toUpperCase())}
                  placeholder="HK0N"
                  className="w-full px-4 py-3 text-2xl font-black tracking-widest text-center rounded-xl border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-800 text-brand-dark dark:text-brand-cream placeholder-slate-300 uppercase focus:outline-none focus:ring-2 focus:ring-brand-active-crimson transition"
                  required
                />
              </div>
            </form>
          </div>

          <button
            form="join-family-form"
            type="submit"
            disabled={joining}
            className="w-full py-2.5 px-4 rounded-xl font-bold text-sm text-white bg-slate-800 dark:bg-slate-700 hover:bg-slate-900 dark:hover:bg-slate-600 transition shadow-md flex items-center justify-center space-x-2 disabled:opacity-50 cursor-pointer"
          >
            <span>
              {joining
                ? isRw
                  ? "Turimo guhuza..."
                  : "Joining..."
                : isRw
                  ? "Injira mu Muryango"
                  : "Join Family"}
            </span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Option 3: Skip for now */}
      <div className="text-center pt-4">
        <button
          type="button"
          onClick={handleSkip}
          className="text-xs font-semibold text-slate-500 hover:text-slate-800 dark:text-slate-400 dark:hover:text-slate-200 underline transition cursor-pointer"
        >
          {isRw
            ? "Komeza kuri Konti Bwite (Simbuka iby'umuryango ubu)"
            : "Skip for now and continue with Personal ledger"}
        </button>
      </div>
    </div>
  );
}
