# UI Context & Design System — Umuryango Ledger

This document establishes the visual design tokens, typography, color architecture, and component standards across GPUI Desktop/Mobile and Next.js Web.

---

## 1. Brand Palette Tokens (10-Color Red Primary System)

The entire design system is anchored in this exact 10-color harmonious palette with **Red** (`#ba181b`) as the main brand primary:

`["#0b090a", "#161a1d", "#660708", "#a4161a", "#ba181b", "#e5383b", "#b1a7a6", "#d3d3d3", "#f5f3f4", "#ffffff"]`

| Token Name | Hex Code | Role in Design |
| :--- | :--- | :--- |
| **`primary`** | `#ba181b` | Main primary buttons, active navigation, brand primary accents, primary indicator. |
| **`primary_hover`** | `#e5383b` | Hover states, active highlights, bright red emphasis, alert accent. |
| **`primary_pressed`** | `#a4161a` | Pressed button states, strong red badges, high-contrast light mode accents. |
| **`primary_deep`** | `#660708` | Deep crimson containers, dark-tinted active pills, dark elevated surfaces. |
| **`dark_canvas`** | `#0b090a` | Night black. Main app background in Dark Mode. |
| **`dark_surface`** | `#161a1d` | Eerie black. Cards, sidebar, modals, inputs in Dark Mode. Secondary text in Light Mode. |
| **`text_muted`** | `#b1a7a6` | Silver chalice. Muted helper labels, subtle borders (at low opacity), neutral indicators. |
| **`light_border`** | `#d3d3d3` | Light gray. Borders, dividers, subtle table headers in Light Mode. |
| **`light_canvas`** | `#f5f3f4` | Cultured off-white. Main app background in Light Mode. Secondary text in Dark Mode. |
| **`light_surface`** | `#ffffff` | Pure white. Cards, modals, inputs in Light Mode. Primary text in Dark Mode. |

### Semantic Status Tokens
- **Success:** Emerald (`#2a9d8f` / `#10b981`) — Positive inflows, completed projects.
- **Warning:** Amber / Coral (`#e76f51` / `#f59e0b`) — Pending custody handovers, upcoming loan deadlines.
- **Error:** Primary Crimson / Imperial Red (`#ba181b` / `#e5383b`) — Overdraft rejections, insufficient custody, failed sync.
- **Info:** Muted Silver / Accent (`#b1a7a6` / `#660708`) — System notices, zero-login status.

---

## 2. Typography: The Archivo Font Family

- **Font Family:** `Archivo`, Google Fonts.
- **Weights:**
  - `400 (Regular)`: Body copy, helper descriptions, notes.
  - `600 (SemiBold)`: Table headers, wallet custodian badges, dates.
  - `700 (Bold)`: Card titles, section headers, buttons.
  - `900 (Black)`: KPI financial balances, page titles, big numbers.
- **Configuration:**
  - In Next.js: Loaded in `web/src/app/layout.tsx` via `next/font/google` (`--font-archivo`).
  - In GPUI: Registered as primary system font in `crates/app-gpui/src/theme.rs`.

---

## 3. Light & Dark Theme Derivation

| Semantic Role | Light Mode | Dark Mode |
| :--- | :--- | :--- |
| **Canvas Background (`bg_canvas`)** | `#f5f3f4` (Light Canvas) | `#0b090a` (Night Black) |
| **Card Surface (`bg_surface`)** | `#ffffff` (Pure White) | `#161a1d` (Eerie Black) |
| **Elevated Surface (`bg_elevated`)** | `#d3d3d3` (Light Border / Header) | `#660708` (Deep Crimson) |
| **Border / Divider (`border_subtle`)** | `#d3d3d3` (Light Gray) | `#b1a7a6` @ 25% opacity |
| **Primary Text (`text_primary`)** | `#0b090a` (Night Black) | `#ffffff` (Pure White) |
| **Secondary Text (`text_secondary`)** | `#161a1d` (Eerie Black) | `#f5f3f4` (Cultured Off-White) |
| **Muted Text (`text_muted`)** | `#b1a7a6` (Silver Chalice) | `#b1a7a6` (Silver Chalice) |
| **Brand Primary (`accent_primary`)** | `#ba181b` (Carmine Red) | `#ba181b` (with `#e5383b` hover) |
| **Brand Secondary (`accent_secondary`)** | `#a4161a` (Blood Red) | `#e5383b` (Imperial Red) |

---

## 4. Component Catalog

### In Rust GPUI (`crates/app-gpui`)
- Uses `gpui-kit` components:
  - `Button`, `Input`, `Modal`, `Dialog`, `Toast`, `Badge`, `Avatar`.
  - Chart components: Donut distribution chart, stacked bar comparison.
  - Sidebar with dynamic collapse into mobile bottom navigation.
  - Settings palette swatches rendering all 10 color tokens with "Reset to Default" action.

### In Next.js Web (`web/src/components/`)
- `ScopeSwitcher`: Toggles between `family` and `personal` scopes.
- `HeaderNav`: Responsive navigation with active route highlights and locale switcher.
- `CustodyChart`: Multi-segment horizontal gauge and allocation breakdown.
- `TransactionModal`: Modal for logging income/expenses with balance verification.
- `HandoverModal`: Custody transfer between family wallets.
- `MothersReportCard`: Print-ready, elder-friendly financial summary.
- `ToastContainer`: Live floating alerts for WebSocket broadcasts.

---

## 5. Unified FontAwesome Vector Icon System (Zero Emojis)

All icons across Desktop, Mobile, and Web use local FontAwesome Free Solid SVGs (`assets/fontawesome/free-solid-svg-icons/`).

- **Rust GPUI (`crates/app-gpui/src/components/icon.rs`):**
  - `AppIcon` enum provides type-safe icons (`Dashboard`, `Transactions`, `Reports`, `Projects`, `Loans`, `FuturePlans`, `Members`, `Settings`, `Bank`, `MobileMoney`, `Cash`, `Income`, `Expense`, `Add`, `Check`, `Sun`, `Moon`, `Bell`, `SyncOnline`, etc.).
  - `app_icon(icon, size, tone, theme) -> impl IntoElement` applies exact palette tokens in Light and Dark modes.
  - Zero emojis allowed anywhere in the GPUI UI hierarchy.
- **Next.js Web (`web/src/components/AppIcon.tsx`):**
  - `<AppIcon name={name} size={size} tone={tone} />` utilizes CSS `mask-image` with `currentColor` fill, adapting seamlessly to active theme mode.
