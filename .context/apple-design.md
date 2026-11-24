# Apple Design Principles & Craftsmanship — Umuryango Ledger

This document establishes the craftsmanship standards inspired by Apple design philosophy: restraint, tactile ergonomics, optical balance, and fluid transitions.

---

## 1. Radical Restraint & Visual Cleanliness

- **Content Precedes Decoration:** UI elements exist to showcase financial reality, not to draw attention to the frame. Avoid heavy gradients, neon accents, or unmotivated visual embellishments.
- **Generous Margins & Optical Balance:** Elements are padded with deliberate optical balance. Text is never jammed against card edges.
- **Sharp, Square Geometry (Zero Border Radius):**
  - All UI surfaces — badges, segmented tabs, inputs, buttons, cards, modals, and spotlight containers — use crisp 0px square corners with no rounded styles.
  - Visual separation is achieved through clean 1px borders, disciplined spacing, and surface contrast rather than corner rounding.

---

## 2. Tactile Ergonomics & Touch Targets

1. **Minimum Touch Target:**
   - Every clickable target (buttons, toggles, table rows, links) must meet the minimum 44×44px hit target on touch devices (`min-h-[44px]`).
2. **Predictable Focus & Active Feedback:**
   - Buttons provide immediate visual confirmation on interaction:
     - `hover:opacity-95` or `hover:bg-slate-100`.
     - `active:scale-[0.98]` tactile press effect.
     - Accessible focus rings: `focus:outline-none focus:ring-2 focus:ring-brand-steel-blue`.

---

## 3. Motion & Transition Standards

- **Short, Intentional Durations:** Animations must never delay the user. Keep durations between **150ms and 250ms**.
- **Timing Curve:** Use `ease-out` or standard cubic-bezier curves for natural deceleration.
- **State Changes:**
  - Light/Dark mode transitions: `transition-colors duration-200`.
  - Modal entrances: Subtle scale up from 95% to 100% with fade: `animate-in fade-in zoom-in-95 duration-150`.
  - Toast alerts: Slide in from the right: `transition-all duration-300 transform translate-y-0`.

---

## 4. Typography Hierarchy & Micro-Copy Craft

- **Crisp Alignment:** All numbers in tables and KPI widgets align to the right to facilitate quick vertical scanning.
- **Monospace and Tabular Numbers:** Use `tabular-nums` for currency values to prevent jitter during real-time balance updates.
- **Natural Micro-Copy:** In Kinyarwanda, write natural, culturally authentic phrasing rather than literal machine-translated English (e.g., use *"Imari Yanjye Bwite"* rather than *"Amafaranga yanjye y'umwimerere"*; use *"Konti yo mu ntoki"* rather than *"Konti y'amafaranga yo mu mufuka"*).
