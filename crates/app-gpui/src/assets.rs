//! Embedded GPUI `AssetSource` serving the app's FontAwesome SVG icons,
//! brand emblem and logo variants, and delegating built-in `gpui-kit` component
//! icons to `gpui_kit::assets::Assets`.

use std::borrow::Cow;

use gpui_kit::{AssetSource, Result, SharedString};

/// Composite asset source for Desktop, Mobile, and WebAssembly builds.
#[derive(Clone, Copy, Debug, Default)]
pub struct AppAssets;

impl AppAssets {
    pub const APP_ICON_PATH: &'static str = "crates/app-gpui/assets/icons/icon-256.png";
    pub const LOGO_PATH: &'static str = "crates/app-gpui/assets/icons/logo-256.png";
    pub const LOGO_BLACK_PATH: &'static str = "crates/app-gpui/assets/icons/logo-black-256.png";
    pub const LOGO_WHITE_PATH: &'static str = "crates/app-gpui/assets/icons/logo-white-256.png";
}

macro_rules! fa_icon {
    ($file:literal) => {
        (
            concat!("crates/app-gpui/assets/icons/fontawesome/", $file),
            include_bytes!(concat!("../assets/icons/fontawesome/", $file)) as &[u8],
        )
    };
}

const EMBEDDED_ASSETS: &[(&str, &[u8])] = &[
    (
        AppAssets::APP_ICON_PATH,
        include_bytes!("../assets/icons/icon-256.png"),
    ),
    (
        AppAssets::LOGO_PATH,
        include_bytes!("../assets/icons/logo-256.png"),
    ),
    (
        AppAssets::LOGO_BLACK_PATH,
        include_bytes!("../assets/icons/logo-black-256.png"),
    ),
    (
        AppAssets::LOGO_WHITE_PATH,
        include_bytes!("../assets/icons/logo-white-256.png"),
    ),
    fa_icon!("arrow-down.svg"),
    fa_icon!("arrow-left.svg"),
    fa_icon!("arrow-right-arrow-left.svg"),
    fa_icon!("arrow-right.svg"),
    fa_icon!("arrow-trend-down.svg"),
    fa_icon!("arrow-trend-up.svg"),
    fa_icon!("arrow-up-from-bracket.svg"),
    fa_icon!("arrow-up.svg"),
    fa_icon!("ban.svg"),
    fa_icon!("bell.svg"),
    fa_icon!("building-columns.svg"),
    fa_icon!("bullseye.svg"),
    fa_icon!("calendar.svg"),
    fa_icon!("camera.svg"),
    fa_icon!("chart-pie.svg"),
    fa_icon!("check.svg"),
    fa_icon!("chevron-down.svg"),
    fa_icon!("chevron-left.svg"),
    fa_icon!("chevron-right.svg"),
    fa_icon!("chevron-up.svg"),
    fa_icon!("circle-check.svg"),
    fa_icon!("circle-exclamation.svg"),
    fa_icon!("circle-info.svg"),
    fa_icon!("circle.svg"),
    fa_icon!("cloud-arrow-up.svg"),
    fa_icon!("copy.svg"),
    fa_icon!("diagram-project.svg"),
    fa_icon!("download.svg"),
    fa_icon!("ellipsis.svg"),
    fa_icon!("eye-slash.svg"),
    fa_icon!("eye.svg"),
    fa_icon!("file-invoice.svg"),
    fa_icon!("filter.svg"),
    fa_icon!("gear.svg"),
    fa_icon!("globe.svg"),
    fa_icon!("grip-vertical.svg"),
    fa_icon!("hand-holding-dollar.svg"),
    fa_icon!("house.svg"),
    fa_icon!("magnifying-glass.svg"),
    fa_icon!("minus.svg"),
    fa_icon!("mobile-screen.svg"),
    fa_icon!("money-bill-wave.svg"),
    fa_icon!("moon.svg"),
    fa_icon!("paperclip.svg"),
    fa_icon!("people-roof.svg"),
    fa_icon!("plus.svg"),
    fa_icon!("rotate.svg"),
    fa_icon!("scale-balanced.svg"),
    fa_icon!("share-nodes.svg"),
    fa_icon!("sliders.svg"),
    fa_icon!("sun.svg"),
    fa_icon!("table-columns.svg"),
    fa_icon!("triangle-exclamation.svg"),
    fa_icon!("users.svg"),
    fa_icon!("wallet.svg"),
    fa_icon!("wifi.svg"),
    fa_icon!("xmark.svg"),
];

fn kit_assets() -> &'static gpui_kit::assets::Assets {
    static KIT_ASSETS: std::sync::OnceLock<gpui_kit::assets::Assets> = std::sync::OnceLock::new();
    KIT_ASSETS.get_or_init(|| gpui_kit::assets::Assets::new(""))
}

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some((_, bytes)) = EMBEDDED_ASSETS.iter().find(|(p, _)| *p == path) {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        kit_assets().load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut results: Vec<SharedString> = EMBEDDED_ASSETS
            .iter()
            .filter(|(p, _)| p.starts_with(path))
            .map(|(p, _)| SharedString::from(*p))
            .collect();
        results.extend(kit_assets().list(path)?);
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::icon::{AppIcon, BrandMark};

    #[test]
    fn loads_eye_and_all_app_icons() {
        let all_icons = [
            AppIcon::Dashboard,
            AppIcon::PersonalWallet,
            AppIcon::FamilyLedger,
            AppIcon::Transactions,
            AppIcon::Reports,
            AppIcon::Handover,
            AppIcon::Projects,
            AppIcon::Loans,
            AppIcon::FuturePlans,
            AppIcon::Members,
            AppIcon::Settings,
            AppIcon::Add,
            AppIcon::Income,
            AppIcon::Expense,
            AppIcon::Share,
            AppIcon::Copy,
            AppIcon::Download,
            AppIcon::UploadReceipt,
            AppIcon::Search,
            AppIcon::Filter,
            AppIcon::Check,
            AppIcon::Close,
            AppIcon::ChevronDown,
            AppIcon::ChevronRight,
            AppIcon::ArrowUp,
            AppIcon::ArrowDown,
            AppIcon::Eye,
            AppIcon::EyeSlash,
            AppIcon::Sun,
            AppIcon::Moon,
            AppIcon::Language,
            AppIcon::Bell,
            AppIcon::SyncOnline,
            AppIcon::SyncOffline,
            AppIcon::SyncPending,
            AppIcon::Info,
            AppIcon::Success,
            AppIcon::Warning,
            AppIcon::Error,
            AppIcon::UpdateAvailable,
            AppIcon::Bank,
            AppIcon::MobileMoney,
            AppIcon::Cash,
        ];

        for icon in all_icons {
            let loaded = AppAssets
                .load(icon.asset_path())
                .expect("asset load should not fail");
            assert!(
                loaded.is_some_and(|b| !b.is_empty()),
                "missing embedded asset for {:?}",
                icon
            );
        }

        for brand in [
            BrandMark::AppIcon,
            BrandMark::Logo,
            BrandMark::LogoBlack,
            BrandMark::LogoWhite,
        ] {
            let loaded = AppAssets
                .load(brand.asset_path())
                .expect("brand asset load should not fail");
            assert!(
                loaded.is_some_and(|b| !b.is_empty()),
                "missing embedded brand asset for {:?}",
                brand
            );
        }
    }
}
