import { existsSync, mkdirSync, copyFileSync } from "fs";
import { join, resolve } from "path";

const SOURCE_DIR = resolve(import.meta.dir, "../../assets/fontawesome/free-solid-svg-icons");
const TARGET_DIR = resolve(import.meta.dir, "../public/icons/fontawesome");

const ICONS = [
  // Navigation & Scopes
  "house.svg",
  "chart-pie.svg",
  "wallet.svg",
  "people-roof.svg",
  "arrow-right-arrow-left.svg",
  "hand-holding-dollar.svg",
  "diagram-project.svg",
  "scale-balanced.svg",
  "bullseye.svg",
  "users.svg",
  "gear.svg",
  "sliders.svg",

  // Actions & Controls
  "plus.svg",
  "arrow-trend-up.svg",
  "arrow-trend-down.svg",
  "share-nodes.svg",
  "copy.svg",
  "download.svg",
  "camera.svg",
  "file-invoice.svg",
  "magnifying-glass.svg",
  "filter.svg",
  "check.svg",
  "xmark.svg",
  "chevron-down.svg",
  "chevron-right.svg",

  // Theme, Language & Status
  "sun.svg",
  "moon.svg",
  "globe.svg",
  "bell.svg",
  "wifi.svg",
  "cloud-arrow-up.svg",
  "ban.svg",
  "rotate.svg",
  "circle-info.svg",
  "circle-check.svg",
  "triangle-exclamation.svg",
  "circle-exclamation.svg",
  "arrow-up-from-bracket.svg",

  // Channels
  "building-columns.svg",
  "mobile-screen.svg",
  "money-bill-wave.svg",
];

if (!existsSync(TARGET_DIR)) {
  mkdirSync(TARGET_DIR, { recursive: true });
}

let copiedCount = 0;
for (const icon of ICONS) {
  const src = join(SOURCE_DIR, icon);
  const dest = join(TARGET_DIR, icon);

  if (existsSync(src)) {
    copyFileSync(src, dest);
    copiedCount++;
  } else {
    console.warn(`[sync-icons] Missing source icon: ${icon}`);
  }
}

console.log(
  `[sync-icons] Successfully synced ${copiedCount} FontAwesome icons to web/public/icons/fontawesome.`,
);
