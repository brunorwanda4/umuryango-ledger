use gpui_kit::prelude::*;
use gpui_kit::{ElementId, IntoElement, ParentElement, Styled, div, px, svg};
use serde::{Deserialize, Serialize};

use crate::theme::SemanticTheme;

/// Type-safe catalog of official FontAwesome icons used across Umuryango Ledger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppIcon {
    // Navigation & Scopes
    Dashboard,
    PersonalWallet,
    FamilyLedger,
    Transactions,
    Reports,
    Handover,
    Projects,
    Loans,
    FuturePlans,
    Members,
    Settings,

    // Actions & Controls
    Add,
    Income,
    Expense,
    Share,
    Copy,
    Download,
    UploadReceipt,
    Search,
    Filter,
    Check,
    Close,
    ChevronDown,
    ChevronRight,
    Eye,
    EyeSlash,

    // Theme, Language & Status
    Sun,
    Moon,
    Language,
    Bell,
    SyncOnline,
    SyncOffline,
    SyncPending,
    Info,
    Success,
    Warning,
    Error,
    UpdateAvailable,

    // Channels
    Bank,
    MobileMoney,
    Cash,
}

impl AppIcon {
    pub const fn file_name(&self) -> &'static str {
        match self {
            AppIcon::Dashboard => "house.svg",
            AppIcon::PersonalWallet => "wallet.svg",
            AppIcon::FamilyLedger => "people-roof.svg",
            AppIcon::Transactions => "arrow-right-arrow-left.svg",
            AppIcon::Reports => "chart-pie.svg",
            AppIcon::Handover => "hand-holding-dollar.svg",
            AppIcon::Projects => "diagram-project.svg",
            AppIcon::Loans => "scale-balanced.svg",
            AppIcon::FuturePlans => "bullseye.svg",
            AppIcon::Members => "users.svg",
            AppIcon::Settings => "gear.svg",
            AppIcon::Add => "plus.svg",
            AppIcon::Income => "arrow-trend-up.svg",
            AppIcon::Expense => "arrow-trend-down.svg",
            AppIcon::Share => "share-nodes.svg",
            AppIcon::Copy => "copy.svg",
            AppIcon::Download => "download.svg",
            AppIcon::UploadReceipt => "camera.svg",
            AppIcon::Search => "magnifying-glass.svg",
            AppIcon::Filter => "filter.svg",
            AppIcon::Check => "check.svg",
            AppIcon::Close => "xmark.svg",
            AppIcon::ChevronDown => "chevron-down.svg",
            AppIcon::ChevronRight => "chevron-right.svg",
            AppIcon::Eye => "eye.svg",
            AppIcon::EyeSlash => "eye-slash.svg",
            AppIcon::Sun => "sun.svg",
            AppIcon::Moon => "moon.svg",
            AppIcon::Language => "globe.svg",
            AppIcon::Bell => "bell.svg",
            AppIcon::SyncOnline => "wifi.svg",
            AppIcon::SyncOffline => "ban.svg",
            AppIcon::SyncPending => "rotate.svg",
            AppIcon::Info => "circle-info.svg",
            AppIcon::Success => "circle-check.svg",
            AppIcon::Warning => "triangle-exclamation.svg",
            AppIcon::Error => "circle-exclamation.svg",
            AppIcon::UpdateAvailable => "arrow-up-from-bracket.svg",
            AppIcon::Bank => "building-columns.svg",
            AppIcon::MobileMoney => "mobile-screen.svg",
            AppIcon::Cash => "money-bill-wave.svg",
        }
    }

    pub const fn asset_path(&self) -> &'static str {
        match self {
            AppIcon::Dashboard => "crates/app-gpui/assets/icons/fontawesome/house.svg",
            AppIcon::PersonalWallet => "crates/app-gpui/assets/icons/fontawesome/wallet.svg",
            AppIcon::FamilyLedger => "crates/app-gpui/assets/icons/fontawesome/people-roof.svg",
            AppIcon::Transactions => {
                "crates/app-gpui/assets/icons/fontawesome/arrow-right-arrow-left.svg"
            }
            AppIcon::Reports => "crates/app-gpui/assets/icons/fontawesome/chart-pie.svg",
            AppIcon::Handover => "crates/app-gpui/assets/icons/fontawesome/hand-holding-dollar.svg",
            AppIcon::Projects => "crates/app-gpui/assets/icons/fontawesome/diagram-project.svg",
            AppIcon::Loans => "crates/app-gpui/assets/icons/fontawesome/scale-balanced.svg",
            AppIcon::FuturePlans => "crates/app-gpui/assets/icons/fontawesome/bullseye.svg",
            AppIcon::Members => "crates/app-gpui/assets/icons/fontawesome/users.svg",
            AppIcon::Settings => "crates/app-gpui/assets/icons/fontawesome/gear.svg",
            AppIcon::Add => "crates/app-gpui/assets/icons/fontawesome/plus.svg",
            AppIcon::Income => "crates/app-gpui/assets/icons/fontawesome/arrow-trend-up.svg",
            AppIcon::Expense => "crates/app-gpui/assets/icons/fontawesome/arrow-trend-down.svg",
            AppIcon::Share => "crates/app-gpui/assets/icons/fontawesome/share-nodes.svg",
            AppIcon::Copy => "crates/app-gpui/assets/icons/fontawesome/copy.svg",
            AppIcon::Download => "crates/app-gpui/assets/icons/fontawesome/download.svg",
            AppIcon::UploadReceipt => "crates/app-gpui/assets/icons/fontawesome/camera.svg",
            AppIcon::Search => "crates/app-gpui/assets/icons/fontawesome/magnifying-glass.svg",
            AppIcon::Filter => "crates/app-gpui/assets/icons/fontawesome/filter.svg",
            AppIcon::Check => "crates/app-gpui/assets/icons/fontawesome/check.svg",
            AppIcon::Close => "crates/app-gpui/assets/icons/fontawesome/xmark.svg",
            AppIcon::ChevronDown => "crates/app-gpui/assets/icons/fontawesome/chevron-down.svg",
            AppIcon::ChevronRight => "crates/app-gpui/assets/icons/fontawesome/chevron-right.svg",
            AppIcon::Eye => "crates/app-gpui/assets/icons/fontawesome/eye.svg",
            AppIcon::EyeSlash => "crates/app-gpui/assets/icons/fontawesome/eye-slash.svg",
            AppIcon::Sun => "crates/app-gpui/assets/icons/fontawesome/sun.svg",
            AppIcon::Moon => "crates/app-gpui/assets/icons/fontawesome/moon.svg",
            AppIcon::Language => "crates/app-gpui/assets/icons/fontawesome/globe.svg",
            AppIcon::Bell => "crates/app-gpui/assets/icons/fontawesome/bell.svg",
            AppIcon::SyncOnline => "crates/app-gpui/assets/icons/fontawesome/wifi.svg",
            AppIcon::SyncOffline => "crates/app-gpui/assets/icons/fontawesome/ban.svg",
            AppIcon::SyncPending => "crates/app-gpui/assets/icons/fontawesome/rotate.svg",
            AppIcon::Info => "crates/app-gpui/assets/icons/fontawesome/circle-info.svg",
            AppIcon::Success => "crates/app-gpui/assets/icons/fontawesome/circle-check.svg",
            AppIcon::Warning => "crates/app-gpui/assets/icons/fontawesome/triangle-exclamation.svg",
            AppIcon::Error => "crates/app-gpui/assets/icons/fontawesome/circle-exclamation.svg",
            AppIcon::UpdateAvailable => {
                "crates/app-gpui/assets/icons/fontawesome/arrow-up-from-bracket.svg"
            }
            AppIcon::Bank => "crates/app-gpui/assets/icons/fontawesome/building-columns.svg",
            AppIcon::MobileMoney => "crates/app-gpui/assets/icons/fontawesome/mobile-screen.svg",
            AppIcon::Cash => "crates/app-gpui/assets/icons/fontawesome/money-bill-wave.svg",
        }
    }
}

/// Type-safe icon dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconSize {
    Xs, // 12px
    Sm, // 14px
    #[default]
    Md, // 16px
    Lg, // 20px
    Xl, // 24px
}

impl IconSize {
    pub const fn px(&self) -> f32 {
        match self {
            IconSize::Xs => 12.0,
            IconSize::Sm => 14.0,
            IconSize::Md => 16.0,
            IconSize::Lg => 20.0,
            IconSize::Xl => 24.0,
        }
    }
}

/// Color tones adapted automatically for Light Mode, Dark Mode, and user theme palettes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconTone {
    #[default]
    Default,
    Muted,
    AccentPrimary,
    AccentSecondary,
    OnPrimaryButton,
    Income,
    Expense,
    Success,
    Warning,
    Error,
    Info,
}

impl IconTone {
    pub fn color(&self, theme: &SemanticTheme) -> gpui_kit::Hsla {
        match self {
            IconTone::Default => theme.text_primary,
            IconTone::Muted => theme.text_muted,
            IconTone::AccentPrimary => theme.accent_primary,
            IconTone::AccentSecondary => theme.accent_secondary,
            IconTone::OnPrimaryButton => theme.bg_canvas,
            IconTone::Income => theme.income_color,
            IconTone::Expense => theme.expense_color,
            IconTone::Success => theme.income_color,
            IconTone::Warning => theme.accent_secondary,
            IconTone::Error => theme.expense_color,
            IconTone::Info => theme.income_color,
        }
    }
}

/// Renders a theme-aware FontAwesome SVG icon
pub fn app_icon(
    icon: AppIcon,
    size: IconSize,
    tone: IconTone,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let size_px = px(size.px());
    let color = tone.color(theme);

    svg()
        .path(icon.asset_path())
        .size(size_px)
        .text_color(color)
}

/// Renders an interactive button containing a theme-aware FontAwesome SVG icon
pub fn icon_button(
    id: impl Into<ElementId>,
    icon: AppIcon,
    size: IconSize,
    tone: IconTone,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let size_px = px(size.px());
    let padding = px(6.0);
    let color = tone.color(theme);

    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .p(padding)
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_elevated))
        .child(
            svg()
                .path(icon.asset_path())
                .size(size_px)
                .text_color(color),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_file_names() {
        assert_eq!(AppIcon::Dashboard.file_name(), "house.svg");
        assert_eq!(AppIcon::PersonalWallet.file_name(), "wallet.svg");
        assert_eq!(AppIcon::FamilyLedger.file_name(), "people-roof.svg");
        assert_eq!(
            AppIcon::Transactions.file_name(),
            "arrow-right-arrow-left.svg"
        );
        assert_eq!(AppIcon::Reports.file_name(), "chart-pie.svg");
        assert_eq!(AppIcon::Handover.file_name(), "hand-holding-dollar.svg");
        assert_eq!(AppIcon::SyncOnline.file_name(), "wifi.svg");
        assert_eq!(AppIcon::SyncOffline.file_name(), "ban.svg");
    }

    #[test]
    fn test_icon_sizes() {
        assert_eq!(IconSize::Xs.px(), 12.0);
        assert_eq!(IconSize::Sm.px(), 14.0);
        assert_eq!(IconSize::Md.px(), 16.0);
        assert_eq!(IconSize::Lg.px(), 20.0);
        assert_eq!(IconSize::Xl.px(), 24.0);
    }
}
