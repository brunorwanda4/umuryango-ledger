//! Native Rust GPUI hooks ported from `web/src/hooks/`.
//!
//! Note: `use-highlighted-code.ts` & `highlighter.tsx` are intentionally omitted
//! because zero installed shadcn components in `web/src/components/ui/` and zero
//! Umuryango Ledger screens depend on Shiki syntax highlighting (documented in
//! `.context/progress-tracker.md` per Rule 1: Zero Dead Code).

pub mod use_mobile;

pub use use_mobile::{
    Breakpoint, MOBILE_BREAKPOINT, TABLET_BREAKPOINT, ViewportMetrics, is_desktop, is_mobile,
    is_tablet, notify_on_resize, observe_viewport, responsive, viewport_height, viewport_width,
};
