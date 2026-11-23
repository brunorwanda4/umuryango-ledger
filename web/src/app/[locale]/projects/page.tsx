export default async function ProjectsPage({ params }: { params: Promise<{ locale: string }> }) {
  const resolvedParams = await params;
  const locale = resolvedParams.locale === "en" ? "en" : "rw";
  const isRw = locale === "rw";

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
          <h1 className="text-3xl font-black text-slate-900 tracking-tight">
            {isRw ? "Imishinga y'Umuryango" : "Family Projects"}
          </h1>
          <p className="text-slate-500 text-sm mt-1">
            {isRw
              ? "Guhuza imbaraga n'umusanzu mu bikorwa by'iterambere ry'umuryango"
              : "Pooled contributions and progress tracking for shared goals"}
          </p>
        </div>

        <button
          type="button"
          className="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-sm rounded-xl shadow-sm transition"
        >
          {isRw ? "+ Tangiza Umushinga Gishya" : "+ Start New Project"}
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {projects.map((proj) => (
          <div
            key={proj.id}
            className="p-6 bg-white rounded-2xl border border-slate-200 shadow-sm space-y-4"
          >
            <div className="flex items-start justify-between">
              <div>
                <h3 className="font-bold text-lg text-slate-900">{proj.name}</h3>
                <p className="text-xs text-slate-500 mt-1">{proj.description}</p>
              </div>
              <span
                className={`px-3 py-1 rounded-full text-xs font-semibold ${
                  proj.percentage === 100
                    ? "bg-emerald-100 text-emerald-800"
                    : "bg-blue-100 text-blue-800"
                }`}
              >
                {proj.status}
              </span>
            </div>

            <div className="space-y-2">
              <div className="flex justify-between text-xs font-semibold">
                <span className="text-slate-500">
                  {isRw ? "Amafaranga amaze kuboneka" : "Progress"}
                </span>
                <span className="text-slate-900">
                  {proj.raised} / {proj.target} ({proj.percentage}%)
                </span>
              </div>
              <div className="w-full bg-slate-100 rounded-full h-2.5 overflow-hidden">
                <div
                  className="bg-emerald-600 h-2.5 rounded-full transition-all"
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
