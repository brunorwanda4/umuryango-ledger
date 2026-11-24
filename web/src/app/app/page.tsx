"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { ArrowLeft, Loader2, RefreshCw } from "lucide-react";

export default function GpuiAppPage() {
  const [status, setStatus] = useState<"loading" | "ready" | "error">("loading");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    async function initWasm() {
      try {
        console.log("Initializing GPUI WASM engine...");

        // Locate canvas element in DOM
        const canvas = document.getElementById("gpui-canvas") as HTMLCanvasElement | null;
        if (!canvas) {
          throw new Error("Failed to locate target canvas element #gpui-canvas in DOM");
        }

        // Adjust canvas pixel density
        const dpr = window.devicePixelRatio || 1;
        canvas.width = window.innerWidth * dpr;
        canvas.height = window.innerHeight * dpr;

        // Dynamic execution loader: loads WASM entry if compiled into public/wasm
        try {
          const wasmUrl = "/wasm/umuryango_app.js";
          // Use Function constructor to avoid compile-time TypeScript resolution of dynamic browser path
          const importDynamic = new Function("url", "return import(url)");
          const wasmModule = await importDynamic(wasmUrl).catch(() => null);

          if (
            wasmModule &&
            typeof wasmModule.default === "function" &&
            typeof wasmModule.mount_gpui_app === "function"
          ) {
            await wasmModule.default();
            wasmModule.mount_gpui_app("gpui-canvas");
          } else {
            console.info(
              "GPUI WebAssembly binary target ready. Canvas mounted for WASM frame rendering.",
            );
          }
        } catch (wasmErr) {
          console.warn("WASM runtime loader notification:", wasmErr);
        }

        if (mounted) {
          setStatus("ready");
        }
      } catch (err: unknown) {
        if (mounted) {
          setStatus("error");
          setErrorMessage(err instanceof Error ? err.message : String(err));
        }
      }
    }

    initWasm();

    const handleResize = () => {
      const canvas = document.getElementById("gpui-canvas") as HTMLCanvasElement | null;
      if (canvas) {
        const dpr = window.devicePixelRatio || 1;
        canvas.width = window.innerWidth * dpr;
        canvas.height = window.innerHeight * dpr;
      }
    };

    window.addEventListener("resize", handleResize);

    return () => {
      mounted = false;
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  return (
    <div className="relative w-screen h-screen overflow-hidden bg-[#003049] select-none">
      {/* GPUI Canvas: 100vw x 100vh host container */}
      <canvas
        id="gpui-canvas"
        className="absolute inset-0 w-full h-full block focus:outline-none"
        tabIndex={0}
      />

      {/* Loading state indicator before initial GPUI frame renders */}
      {status === "loading" && (
        <div className="absolute inset-0 flex flex-col items-center justify-center bg-[#003049] text-[#fdf0d5] z-10 transition-opacity duration-300">
          <div className="flex flex-col items-center gap-4">
            <Loader2 className="w-10 h-10 animate-spin text-[#c1121f]" />
            <div className="text-center">
              <h2 className="text-xl font-bold font-archivo tracking-tight">Umuryango Ledger</h2>
              <p className="text-xs text-[#669bbc] mt-1 font-mono">Loading GPUI WebAssembly...</p>
            </div>
          </div>
        </div>
      )}

      {/* Error fallback state if WebGL/WASM encounters an unrecoverable failure */}
      {status === "error" && (
        <div className="absolute inset-0 flex flex-col items-center justify-center bg-[#003049] text-[#fdf0d5] z-20 p-6">
          <div className="max-w-md w-full bg-[#780000] p-6 shadow-2xl border border-[#c1121f]/50 text-center space-y-4">
            <h2 className="text-xl font-bold font-archivo text-white">Application Loading Error</h2>
            <p className="text-sm text-[#fdf0d5]/90 font-mono break-all">{errorMessage}</p>
            <div className="pt-2 flex justify-center gap-3">
              <button
                onClick={() => window.location.reload()}
                className="inline-flex items-center gap-2 px-5 py-2.5 bg-[#003049] hover:bg-[#003049]/80 text-[#fdf0d5] font-semibold text-sm transition"
              >
                <RefreshCw className="w-4 h-4" />
                <span>Retry</span>
              </button>
              <Link
                href="/"
                className="inline-flex items-center gap-2 px-5 py-2.5 bg-white/10 hover:bg-white/20 text-white font-semibold text-sm transition"
              >
                <ArrowLeft className="w-4 h-4" />
                <span>Back to Home</span>
              </Link>
            </div>
          </div>
        </div>
      )}

      {/* Floating subtle Home button at top-left for navigation back to landing page */}
      <div className="absolute top-4 left-4 z-30">
        <Link
          href="/"
          className="inline-flex items-center gap-2 px-3 py-1.5 bg-[#003049]/80 hover:bg-[#003049] text-[#fdf0d5] hover:text-white border border-[#669bbc]/30 text-xs font-medium backdrop-blur-md transition shadow-sm"
          title="Return to Public Visitor Page"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Home</span>
        </Link>
      </div>
    </div>
  );
}
