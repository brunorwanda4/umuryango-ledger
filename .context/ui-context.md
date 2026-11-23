# UI Context & Design System — Umuryango Ledger

This document establishes the visual design tokens, typography, color architecture, and component standards across GPUI Desktop/Mobile and Next.js Web.

---

## 1. Brand Palette Tokens

The entire design system is anchored in five harmonious brand tokens:

| Token Name | Hex Code | Role in Design |
| :--- | :--- | :--- |
| **Primary Dark** | `#003049` | Prussian Navy. Dark canvas background, primary brand headers, bold text in light mode, primary container surfaces. |
| **Accent Cream** | `#fdf0d5` | Warm Cream. Light canvas background, surface contrast for badges, cards, and warm highlights. |
| **Primary Crimson** | `#780000` | Deep Crimson. High-importance alerts, warning states, primary destructive actions, personal scope accents. |
| **Active Crimson** | `#c1121f` | Vibrant Red. Active badges, critical buttons, outflow indicator, high-visibility badges. |
| **Steel Blue** | `#669bbc` | Balanced Cerulean. Secondary actions, chart progress bars, inflows, link accents. |

### Semantic Status Tokens
- **Success:** Emerald (`#10b981` / `#059669`) — Positive inflows, completed projects.
- **Warning:** Amber (`#f59e0b` / `#d97706`) — Pending custody handovers, upcoming loan deadlines.
- **Error:** Crimson (`#c1121f` / `#780000`) — Overdraft rejections, insufficient custody, failed sync.
- **Info:** Steel Blue (`#669bbc` / `#0284c7`) — System notices, zero-login status.

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
| **Canvas Background** | `#f8fafc` (Slate 50) | `#001e2e` (Deep Prussian Dark) |
| **Card Surface** | `#ffffff` | `#003049` (Primary Dark) |
| **Border / Divider** | `#e2e8f0` (Slate 200) | `#0b4566` (Muted Steel Navy) |
| **Primary Text** | `#0f172a` (Slate 900) | `#f8fafc` (Slate 50) |
| **Muted Text** | `#64748b` (Slate 500) | `#94a3b8` (Slate 400) |
| **Accent Highlights** | `#fdf0d5` (Accent Cream) | `#669bbc` (Steel Blue) |

---

## 4. Component Catalog

### In Rust GPUI (`crates/app-gpui`)
- Uses `gpui-kit` components:
  - `Button`, `Input`, `Modal`, `Dialog`, `Toast`, `Badge`, `Avatar`.
  - Chart components: Donut distribution chart, stacked bar comparison.
  - Sidebar with dynamic collapse into mobile bottom navigation.

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
