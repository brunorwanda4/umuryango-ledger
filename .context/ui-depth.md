# UI Depth, Hierarchy & Responsive Adaptation — Umuryango Ledger

This document establishes the depth hierarchy, elevation, color contrast, and responsive layout rules across form factors.

---

## 1. Visual Hierarchy & Contrast Rules

### Light Mode Contrast
- **Canvas (`#f8fafc`):** Very light slate neutral background.
- **Surface Cards (`#ffffff`):** Pure white with subtle 1px border (`#e2e8f0`) and soft elevation shadow (`shadow-sm`).
- **Accent Cards (`#fdf0d5`):** Warm cream surface used sparingly to highlight family identity, spotlight boxes, or pending approval banners.
- **Deep Navy Text (`#0f172a`):** Maximum readability against white and cream surfaces.

### Dark Mode Contrast
- **Canvas (`#001e2e`):** Deep prussian dark background, preserving battery on OLED mobile screens.
- **Surface Cards (`#003049`):** Primary navy card surface with sharp subtle borders (`#0b4566`).
- **Text (`#f8fafc`):** High contrast white/off-white text with secondary metadata in `#94a3b8`.
- **Accent Elements (`#fdf0d5` & `#669bbc`):** Cream and steel blue accents provide luminous contrast without eye strain.

---

## 2. Spacing Scale & Elevation Layers

We use a disciplined 4px / 8px scale:
- `p-1 / gap-1`: 4px — Tight segmented controls (`ScopeSwitcher`).
- `p-2 / gap-2`: 8px — Badge padding, button icons.
- `p-3 / gap-3`: 12px — Compact grid items, custody sub-cards.
- `p-4 / gap-4`: 16px — Standard card internal padding, table cells.
- `p-6 / gap-6`: 24px — Container headers, modals, feature cards.
- `p-8 / gap-8`: 32px — Big spotlight cards (Mother's Report header).

### Elevation System
- **Layer 0 (Canvas):** Flat background (`bg-slate-50` / `bg-[#001e2e]`).
- **Layer 1 (Cards):** Bordered surface + `shadow-sm`.
- **Layer 2 (Modals & Drawers):** Backdrop blur (`backdrop-blur-sm`) + `shadow-2xl` + `z-50`.
- **Layer 3 (Toasts & Overlays):** Floating top-right notifications with `shadow-xl` + `z-50`.

---

## 3. Responsive Layout Transitions

### Desktop (Viewport Width >= 1024px)
- Top navigation bar or side navigation drawer with full labels.
- 4-column KPI cards (`grid-cols-4`).
- Multi-column transaction tables with full date, description, category, wallet, actor, and amount.
- Quick action modals centered over a shaded backdrop.

### Tablet (Viewport Width 768px – 1023px)
- 2-column KPI cards (`grid-cols-2`).
- Responsive table with horizontal scroll or prioritized primary columns.

### Mobile (Viewport Width < 768px)
- Navigation shifts to fixed top bar + bottom quick action bar.
- KPI cards stack cleanly in 1 column or 2 compact columns.
- Modals slide up as bottom sheets (`animate-in slide-in-from-bottom`).
- Tables collapse into clean card rows displaying actor, amount, and wallet badge.

---

## 4. Mobile Safe Area & Window Insets (Android & iOS)

### System Insets Architecture
Edge-to-edge rendering requires handling system intrusions gracefully:
- **Top Inset (`safe-area-inset-top`):** Accommodates the Android status bar (clock, battery, signal), camera notches, and dynamic island cutouts (typically 24dp - 48dp).
- **Bottom Inset (`safe-area-inset-bottom`):** Accommodates the Android system navigation bar (3-button navigation bar 48dp - 56dp, or gesture navigation bar 16dp - 24dp) and soft keyboard (IME: 240dp - 300dp).
- **Horizontal Insets (`safe-area-inset-left / right`):** Accommodates landscape display cutouts.

### Implementation Guidelines
1. **Rust GPUI (`crates/app-gpui`):**
   - `SafeAreaInsets` struct in `src/safe_area.rs` encapsulates top, bottom, left, right, and IME insets.
   - `SafeAreaView` in `src/components/safe_area.rs` provides a declarative padding wrapper.
   - Screen root containers apply `.pb(px(24.0) + state.safe_area.total_bottom_px())` so lists and action buttons are never obscured by Android navigation bars.
   - Android edge-to-edge system bars contrast is updated dynamically via `set_system_bars_contrast(is_light)`.
2. **Next.js Web (`web/`):**
   - `viewportFit: "cover"` in `web/src/app/layout.tsx`.
   - Utility classes `.pt-safe`, `.pb-safe`, `.pl-safe`, `.pr-safe`, `.mt-safe`, `.mb-safe` in `web/src/app/globals.css`.
   - Headers use `.pt-safe` and mobile sheets/footers use `.pb-safe`.
