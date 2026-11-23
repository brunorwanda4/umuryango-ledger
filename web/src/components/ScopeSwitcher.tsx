"use client";

import React, { useState } from "react";

interface ScopeSwitcherProps {
  currentScope?: "personal" | "family";
  locale?: "rw" | "en";
  onScopeChange?: (scope: "personal" | "family") => void;
}

export const ScopeSwitcher: React.FC<ScopeSwitcherProps> = ({
  currentScope = "family",
  locale = "rw",
  onScopeChange,
}) => {
  const [scope, setScope] = useState<"personal" | "family">(currentScope);

  const handleToggle = (nextScope: "personal" | "family") => {
    setScope(nextScope);
    onScopeChange?.(nextScope);
  };

  const personalLabel = locale === "rw" ? "Imari Yanjye Bwite" : "Personal Finances";
  const familyLabel = locale === "rw" ? "Igitabo cy'Umuryango" : "Family Ledger";

  return (
    <div className="inline-flex p-1 bg-slate-100 rounded-xl border border-slate-200">
      <button
        type="button"
        onClick={() => handleToggle("family")}
        className={`px-4 py-1.5 rounded-lg text-sm font-semibold transition-all ${
          scope === "family"
            ? "bg-emerald-600 text-white shadow-sm"
            : "text-slate-600 hover:text-slate-900"
        }`}
      >
        🏛️ {familyLabel}
      </button>

      <button
        type="button"
        onClick={() => handleToggle("personal")}
        className={`px-4 py-1.5 rounded-lg text-sm font-semibold transition-all ${
          scope === "personal"
            ? "bg-slate-800 text-white shadow-sm"
            : "text-slate-600 hover:text-slate-900"
        }`}
      >
        🔒 {personalLabel}
      </button>
    </div>
  );
};
