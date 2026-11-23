mod components;
mod mobile;
mod safe_area;
mod screens;
mod state;
mod theme;
pub mod updater;

use gpui_kit::prelude::*;
use gpui_kit::{div, px, Context, IntoElement, ParentElement, Render, Styled, Window};
pub use safe_area::SafeAreaInsets;
use state::{AppState, Screen};
use theme::archivo_font_bytes;
use umuryango_api_client::{RealtimeClient, RealtimeHandle};
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_storage_local::LocalDatabase;

/// Official 256x256 embedded emblem icon
pub const APP_ICON_256_PNG: &[u8] = include_bytes!("../assets/icons/icon-256.png");

struct RootView {
    state: AppState,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme.clone();
        let screen_element = match self.state.current_screen {
            Screen::Dashboard => {
                screens::DashboardScreen::render("screen-dashboard", &self.state, &theme)
                    .into_any_element()
            }
            Screen::Transactions => {
                screens::TransactionsScreen::render("screen-txs", &self.state, &theme)
                    .into_any_element()
            }
            Screen::Reports => {
                screens::ReportViewScreen::render("screen-reports", &self.state, &theme)
                    .into_any_element()
            }
            Screen::Projects => {
                screens::ProjectsScreen::render("screen-projects", &self.state, &theme)
                    .into_any_element()
            }
            Screen::Loans => {
                screens::LoansScreen::render("screen-loans", &self.state, &theme).into_any_element()
            }
            Screen::Plans => {
                screens::FuturePlansScreen::render("screen-plans", &self.state, &theme)
                    .into_any_element()
            }
            Screen::Members => {
                screens::MembersScreen::render("screen-members", &self.state, &theme)
                    .into_any_element()
            }
            Screen::Settings => {
                screens::SettingsScreen::render("screen-settings", &self.state, &theme)
                    .into_any_element()
            }
        };

        let top_safe_area = self.state.safe_area.top_px();

        div()
            .flex()
            .flex_row()
            .w_full()
            .h_full()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Desktop Sidebar Navigation
            .child(components::Sidebar::render(
                "app-sidebar",
                &self.state,
                &theme,
            ))
            // Active Screen Content Area with Update Banner & Floating Toast
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .relative()
                    .children(updater::update_banner::UpdateBanner::render(
                        "app-update-banner",
                        &self.state.updater.status,
                        self.state.locale,
                        &theme,
                    ))
                    .child(div().flex_1().w_full().h_full().child(screen_element))
                    .children(if let Some((msg, is_error)) = &self.state.toast_message {
                        let bg = if *is_error {
                            theme.expense_color
                        } else {
                            theme.bg_elevated
                        };
                        let border = if *is_error {
                            theme.expense_color
                        } else {
                            theme.income_color
                        };
                        Some(
                            div()
                                .absolute()
                                .top(px(16.0) + top_safe_area)
                                .right(px(16.0))
                                .p(px(12.0))
                                .rounded(px(8.0))
                                .bg(bg)
                                .border_1()
                                .border_color(border)
                                .text_xs()
                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                .text_color(theme.text_primary)
                                .child(format!("[Notice] {}", msg)),
                        )
                    } else {
                        None
                    }),
            )
            // Realtime Notifications Drawer
            .children(if self.state.show_notifications_drawer {
                Some(components::NotificationsDrawer::render(
                    "app-notifications-drawer",
                    &self.state,
                    &theme,
                ))
            } else {
                None
            })
    }
}

pub fn spawn_realtime_listener(
    ws_url: String,
    token: String,
) -> (
    RealtimeHandle,
    tokio::sync::mpsc::UnboundedReceiver<RealtimeEvent>,
) {
    RealtimeClient::connect(ws_url, token)
}

fn main() {
    println!("Initializing Umuryango Ledger GPUI desktop/mobile application...");
    let db =
        LocalDatabase::open_default().unwrap_or_else(|_| LocalDatabase::open_in_memory().unwrap());
    let mut state = AppState::default();
    state.load_from_db(&db);

    println!(
        "Active scope: {}, Locale: {:?}, Theme: {:?}",
        state.active_scope, state.locale, state.theme_mode
    );

    let app = gpui_kit::application();
    app.run(move |cx| {
        if let Err(e) = cx.text_system().add_fonts(archivo_font_bytes()) {
            eprintln!("Warning: Failed to load bundled Archivo fonts: {:?}", e);
        }

        let mut options = gpui_kit::WindowOptions::default();
        options.titlebar = Some(gpui_kit::TitlebarOptions {
            title: Some("Umuryango Ledger".into()),
            ..Default::default()
        });
        let _ = cx.open_window(options, |_window, cx| cx.new(|_| RootView { state }));
    });
}
