import React from "react";
import type { Metadata, Viewport } from "next";
import { Archivo } from "next/font/google";
import "@/app/globals.css";

const archivo = Archivo({
  subsets: ["latin"],
  variable: "--font-archivo",
  display: "swap",
});

export const metadata: Metadata = {
  title: "Umuryango Ledger - Family & Personal Financial Accountability",
  description: "Multi-tenant bilingual family and personal finance accountability platform.",
  icons: {
    icon: "/favicon.ico",
    shortcut: "/favicon.ico",
    apple: "/apple-touch-icon.png",
  },
  manifest: "/manifest.json",
};

export const viewport: Viewport = {
  themeColor: "#003049",
  width: "device-width",
  initialScale: 1,
  viewportFit: "cover",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html className={archivo.variable}>
      <body className="min-h-screen bg-slate-50 dark:bg-[#001e2e] text-slate-900 dark:text-slate-100 font-sans antialiased selection:bg-brand-active-crimson selection:text-white transition-colors duration-200">
        {children}
      </body>
    </html>
  );
}
