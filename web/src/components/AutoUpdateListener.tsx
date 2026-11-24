"use client";

import React, { useEffect, useState, useCallback } from "react";
import { RefreshCw, AlertCircle, X } from "lucide-react";

interface AutoUpdateListenerProps {
  locale: string;
}

interface ReleaseCheckResult {
  update_available: boolean;
  is_mandatory: boolean;
  latest_version: string;
  download_url: string;
  release_notes_en: string;
  release_notes_rw: string;
  latest_bundle_version: number;
}

export function AutoUpdateListener({ locale }: AutoUpdateListenerProps) {
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [isMandatory, setIsMandatory] = useState(false);
  const [newVersion, setNewVersion] = useState<string>("");
  const [waitingWorker, setWaitingWorker] = useState<ServiceWorker | null>(null);
  const [dismissed, setDismissed] = useState(false);

  const isRw = locale === "rw";

  const applyUpdate = useCallback(() => {
    if (waitingWorker) {
      waitingWorker.postMessage({ type: "SKIP_WAITING" });
    }
    window.location.reload();
  }, [waitingWorker]);

  // 1. Service Worker registration & lifecycle listener
  useEffect(() => {
    if (typeof window === "undefined" || !("serviceWorker" in navigator)) {
      return;
    }

    navigator.serviceWorker
      .register("/sw.js")
      .then((reg) => {
        if (reg.waiting) {
          setWaitingWorker(reg.waiting);
          setUpdateAvailable(true);
        }

        reg.addEventListener("updatefound", () => {
          const installingWorker = reg.installing;
          if (!installingWorker) return;

          installingWorker.addEventListener("statechange", () => {
            if (installingWorker.state === "installed" && navigator.serviceWorker.controller) {
              setWaitingWorker(installingWorker);
              setUpdateAvailable(true);
            }
          });
        });
      })
      .catch((err) => {
        console.debug("ServiceWorker registration skipped or failed:", err);
      });

    let refreshing = false;
    navigator.serviceWorker.addEventListener("controllerchange", () => {
      if (!refreshing) {
        refreshing = true;
        window.location.reload();
      }
    });
  }, []);

  // 2. Poll Backend Release API & OTA Bundle
  useEffect(() => {
    let isMounted = true;

    async function checkReleaseApi() {
      try {
        const res = await fetch("/api/v1/releases/check?platform=WEB&current_version=0.1.0");
        if (!res.ok) return;

        const data: ReleaseCheckResult = await res.json();
        if (data.update_available && isMounted) {
          setUpdateAvailable(true);
          setIsMandatory(data.is_mandatory);
          setNewVersion(data.latest_version);
        }
      } catch {
        // Offline or backend unavailable, continue running cached version
      }
    }

    checkReleaseApi();
    const interval = setInterval(checkReleaseApi, 60_000); // Check every minute

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, []);

  if (!updateAvailable || (dismissed && !isMandatory)) {
    return null;
  }

  const titleText = isMandatory
    ? isRw
      ? "Iri vugurura ni ngombwa. Vugurura ubu."
      : "This update is required. Please update now."
    : isRw
      ? `Verisiyo nshya ${newVersion ? `(${newVersion}) ` : ""}iraboneka.`
      : `A new version ${newVersion ? `(${newVersion}) ` : ""}is ready.`;

  const buttonText = isRw ? "Vugurura Ubu" : "Update Now";

  return (
    <div
      role="alert"
      className={`w-full py-2.5 px-4 sm:px-6 flex items-center justify-between transition-colors shadow-sm ${
        isMandatory
          ? "bg-[#780000] text-[#fdf0d5]"
          : "bg-[#003049] text-[#fdf0d5] border-b border-[#669bbc]/30"
      }`}
    >
      <div className="flex items-center gap-3">
        {isMandatory ? (
          <AlertCircle className="w-5 h-5 text-[#fdf0d5] shrink-0" />
        ) : (
          <RefreshCw className="w-4 h-4 text-[#669bbc] shrink-0 animate-spin" />
        )}
        <p className="text-xs sm:text-sm font-medium">{titleText}</p>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={applyUpdate}
          className="px-3 py-1 text-xs font-semibold bg-[#fdf0d5] text-[#003049] hover:bg-white transition-colors cursor-pointer"
        >
          {buttonText}
        </button>

        {!isMandatory && (
          <button
            onClick={() => setDismissed(true)}
            aria-label="Dismiss banner"
            className="p-1 text-[#fdf0d5]/80 hover:text-[#fdf0d5] transition-colors cursor-pointer"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
}
