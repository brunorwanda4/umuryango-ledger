"use client";

import React, { use } from "react";
import { useLedger } from "@/lib/LedgerContext";

export default function ProjectsPage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = use(params);
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

  const { scope } = useLedger();

  const projects = [
    {
      id: "proj-1",
      name: isRw ? "Ubwubatsi bw'Inzu y'Ababyeyi" : "Parents' House Renovation",
      description: isRw
        ? "Gusana igisenge no gushyiramo sima mu nzu y'ababyeyi mu cyaro"
        : "Roof replacement and floor cementing for rural family homestead",
      target: "5,000,000 RWF",
      raised: "1,200,000 RWF",
      percentage: 24,
      status: isRw ? "Iri Gukorwa" : "In Progress",
    },
    {
      id: "proj-2",
      name: isRw ? "Minerval y'Abana (Igihembwe cya 1)" : "School Fees (Term 1)",
      description: isRw
        ? "Kwishyurira abana bose b'umuryango amashuri yisumbuye"
        : "Pooled school tuition funds for secondary students",
      target: "1,800,000 RWF",
      raised: "1,800,000 RWF",
      percentage: 100,
      status: isRw ? "Byarangiye" : "Completed",
    },
  ];

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div>
          <div className="inline-block px-2.5 py-0.5 rounded-full text-[10px] font-extrabold uppercase bg-brand-cream/80 dark:bg-slate-800 text-brand-dark dark:text-brand-cream mb-1">
            {scope === "family"
              ? isRw
                ? "🏛️ Imishinga y'Umuryango"
                : "🏛️ Family Projects"
              : isRw
                ? "🔒 Imishinga Bwite"
                : "🔒 Personal Goals"}
          </div>
          <h1 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight">
            {isRw ? "Imishinga y'Umuryango" : "Family Projects"}
          </h1>
          <p className="text-slate-500 dark:text-slate-400 text-sm mt-1">
            {isRw
              ? "Guhuza imbaraga n'umusanzu mu bikorwa by'iterambere ry'umuryango"
              : "Pooled contributions and progress tracking for shared goals"}
          </p>
        </div>

        <button
          type="button"
          className="px-4 py-2 bg-brand-dark dark:bg-brand-active-crimson text-white font-bold text-xs rounded-xl shadow-md transition"
        >
          {isRw ? "+ Tangiza Umushinga Mushya" : "+ Start New Project"}
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {projects.map((proj) => (
          <div
            key={proj.id}
            className="p-6 bg-white dark:bg-brand-dark rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm space-y-4 transition-colors"
          >
            <div className="flex items-start justify-between">
              <div>
                <h3 className="font-bold text-base text-slate-900 dark:text-white">{proj.name}</h3>
                <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                  {proj.description}
                </p>
              </div>
              <span
                className={`px-3 py-1 rounded-full text-[11px] font-bold ${
                  proj.percentage === 100
                    ? "bg-emerald-100 text-emerald-800 dark:bg-emerald-900/40 dark:text-emerald-300"
                    : "bg-brand-cream/80 text-brand-dark dark:bg-slate-800 dark:text-brand-cream"
                }`}
              >
                {proj.status}
              </span>
            </div>

            <div className="space-y-2">
              <div className="flex justify-between text-xs font-semibold">
                <span className="text-slate-500 dark:text-slate-400">
                  {isRw ? "Amafaranga amaze kuboneka" : "Progress"}
                </span>
                <span className="text-slate-900 dark:text-white font-bold">
                  {proj.raised} / {proj.target} ({proj.percentage}%)
                </span>
              </div>
              <div className="w-full bg-slate-100 dark:bg-slate-800 rounded-full h-3 overflow-hidden">
                <div
                  className="bg-brand-steel-blue h-3 rounded-full transition-all duration-500"
                  style={{ width: `${proj.percentage}%` }}
                />
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
