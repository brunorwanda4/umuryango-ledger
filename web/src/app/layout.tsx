import React from "react";
import type { Metadata, Viewport } from "next";
import { Archivo, Nunito_Sans } from "next/font/google";
import "@/app/globals.css";
import { cn } from "@/lib/utils";

const nunitoSans = Nunito_Sans({ subsets: ["latin"], variable: "--font-sans" });

const archivo = Archivo({
  subsets: ["latin"],
  variable: "--font-archivo",
  display: "swap",
});

export const metadata: Metadata = {
  title: "Umuryango Ledger - Family & Personal Financial Accountability",
  description: "Multi-tenant bilingual family and personal finance accountability platform.",
  icons: {
    icon: [
      { url: "/favicon.ico", sizes: "any" },
      { url: "/icon-192.png", type: "image/png", sizes: "192x192" },
      { url: "/icon-512.png", type: "image/png", sizes: "512x512" },
    ],
    shortcut: "/favicon.ico",
    apple: [{ url: "/apple-touch-icon.png", sizes: "180x180", type: "image/png" }],
  },
  manifest: "/manifest.json",
};

export const viewport: Viewport = {
  themeColor: "#0b090a",
  width: "device-width",
  initialScale: 1,
  viewportFit: "cover",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html className={cn("font-sans", nunitoSans.variable, archivo.variable)}>
      <body className="min-h-screen bg-slate-50 dark:bg-[#001e2e] text-slate-900 dark:text-slate-100 font-sans antialiased selection:bg-brand-active-crimson selection:text-white transition-colors duration-200">
        {children}
      </body>
    </html>
  );
}
