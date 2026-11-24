mod actions_auth;
mod actions_ledger;
pub mod api_bridge;
pub mod assets;
pub mod common;
pub mod components;
pub mod db_facade;
pub mod hooks;
pub mod mobile;
pub mod safe_area;
pub mod screens;
pub mod state;
pub mod theme;
pub mod updater;
pub mod web;
pub use web::mount_gpui_app;

use std::sync::{Arc, Mutex};

use api_bridge::ApiBridge;
use components::entity_modal_inputs::EntityModalInputs;
use components::entity_modals::EntityModalDialog;
use components::icon::{AppIcon, IconSize, IconTone, app_icon};
use components::quick_entry::{QuickEntryDialog, QuickEntryForm};
use components::quick_entry_inputs::QuickEntryInputs;
use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window, div, hsla, px,
};
pub use safe_area::SafeAreaInsets;
use screens::auth_inputs::AuthInputs;
use screens::onboarding_inputs::OnboardingInputs;
use state::{AppState, Screen};
use theme::ThemeMode;
#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::{RealtimeClient, RealtimeHandle};
#[cfg(not(target_arch = "wasm32"))]
use umuryango_core_domain::realtime::RealtimeEvent;

/// Official 256x256 embedded emblem icon
pub const APP_ICON_256_PNG: &[u8] = include_bytes!("../assets/icons/icon-256.png");

pub struct RootView {
    pub state: AppState,
    pub db: Option<Arc<Mutex<crate::db_facade::LocalDatabase>>>,
    pub api: ApiBridge,
    pub quick_entry_form: QuickEntryForm,
    pub auth_inputs: AuthInputs,
    pub onboarding_inputs: OnboardingInputs,
    pub quick_entry_inputs: QuickEntryInputs,
    pub entity_modal_inputs: EntityModalInputs,
}

impl RootView {
    pub fn new(
        state: AppState,
        db: Option<Arc<Mutex<crate::db_facade::LocalDatabase>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_with_db(state, db, window, cx)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_wasm(state: AppState, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::new_with_db(state, None, window, cx)
    }

    fn new_with_db(
        mut state: AppState,
        db: Option<Arc<Mutex<crate::db_facade::LocalDatabase>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let restored_token = db
            .as_ref()
            .and_then(|d| d.lock().ok())
            .and_then(|locked| state.restore_session_from_db(&locked));

        let api = ApiBridge::new(state.locale);
        state.api_base_url = api.base_url();
        state.theme.apply_to_kit(cx);

        let form = QuickEntryForm {
            source_wallet_id: state.active_wallets().first().map(|w| w.id),
            destination_wallet_id: state.active_wallets().get(1).map(|w| w.id),
            ..Default::default()
        };

        let locale = state.locale;
        let auth_inputs = AuthInputs::new(window, cx);
        let onboarding_inputs = OnboardingInputs::new(locale, window, cx);
        let quick_entry_inputs = QuickEntryInputs::new(window, cx);
        let entity_modal_inputs = EntityModalInputs::new(window, cx);

        #[allow(unused_mut)]
        let mut view = Self {
            state,
            db,
            api,
            quick_entry_form: form,
            auth_inputs,
            onboarding_inputs,
            quick_entry_inputs,
            entity_modal_inputs,
        };

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(token) = restored_token {
            view.verify_restored_session(token, cx);
        }

        view
    }

    pub fn select_screen(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.state.select_screen(screen);
        cx.notify();
    }

    pub fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
        self.state.toggle_theme(db_lock.as_deref());
        self.state.theme.apply_to_kit(cx);
        cx.notify();
    }

    pub fn set_theme_mode(&mut self, is_dark: bool, cx: &mut Context<Self>) {
        let target_mode = if is_dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        if self.state.theme_mode != target_mode {
            let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
            self.state.set_theme_mode(target_mode, db_lock.as_deref());
            self.state.theme.apply_to_kit(cx);
            cx.notify();
        }
    }

    pub fn reset_theme_palette(&mut self, cx: &mut Context<Self>) {
        let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
        self.state
            .set_theme_config(crate::theme::ThemeConfig::default(), db_lock.as_deref());
        self.state.theme.apply_to_kit(cx);
        cx.notify();
    }

    pub fn toggle_notifications(&mut self, cx: &mut Context<Self>) {
        self.state.toggle_notifications_drawer();
        cx.notify();
    }

    pub fn close_notifications(&mut self, cx: &mut Context<Self>) {
        self.state.show_notifications_drawer = false;
        cx.notify();
    }

    pub fn mark_all_notifications_read(&mut self, cx: &mut Context<Self>) {
        self.state.mark_all_notifications_as_read();
        cx.notify();
    }

    pub fn dismiss_toast(&mut self, cx: &mut Context<Self>) {
        self.state.clear_toast();
        cx.notify();
    }

    pub fn copy_to_clipboard(&mut self, text: String, feedback: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(gpui_kit::ClipboardItem::new_string(text));
        self.state.set_toast(feedback, false);
        cx.notify();
    }

    fn render_toast(
        &self,
        theme: &theme::SemanticTheme,
        top_safe_area: gpui_kit::Pixels,
        cx: &Context<Self>,
    ) -> Option<impl IntoElement> {
        let (msg, is_error) = self.state.toast_message.as_ref()?;
        let bg = if *is_error {
            theme.expense_color.opacity(0.15)
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
                .id("app-toast-dismiss")
                .cursor_pointer()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.dismiss_toast(cx);
                }))
                .absolute()
                .top(px(16.0) + top_safe_area)
                .right(px(16.0))
                .max_w(px(420.0))
                .flex()
                .items_center()
                .gap(px(8.0))
                .p(px(12.0))
                .bg(bg)
                .border_1()
                .border_color(border)
                .text_xs()
                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .child(app_icon(
                    if *is_error {
                        AppIcon::Error
                    } else {
                        AppIcon::Success
                    },
                    IconSize::Sm,
                    if *is_error {
                        IconTone::Expense
                    } else {
                        IconTone::Income
                    },
                    theme,
                ))
                .child(msg.clone()),
        )
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme.clone();
        let screen_element = match self.state.current_screen {
            Screen::Auth => screens::AuthScreen::render(
                "screen-auth",
                &self.state,
                &self.auth_inputs,
                &theme,
                cx,
            )
            .into_any_element(),
            Screen::Onboarding => screens::OnboardingScreen::render(
                "screen-onboarding",
                &self.state,
                &self.onboarding_inputs,
                &theme,
                cx,
            )
            .into_any_element(),
            Screen::Dashboard => {
                screens::DashboardScreen::render("screen-dashboard", &self.state, &theme, cx)
                    .into_any_element()
            }
            Screen::Transactions => {
                screens::TransactionsScreen::render("screen-txs", &self.state, &theme, cx)
                    .into_any_element()
            }
            Screen::Reports => {
                screens::ReportViewScreen::render("screen-reports", &self.state, &theme, cx)
                    .into_any_element()
            }
            Screen::Projects => {
                screens::ProjectsScreen::render("screen-projects", &self.state, &theme, cx)
                    .into_any_element()
            }
            Screen::Loans => screens::LoansScreen::render("screen-loans", &self.state, &theme, cx)
                .into_any_element(),
            Screen::Plans => {
                screens::FuturePlansScreen::render("screen-plans", &self.state, &theme, cx)
                    .into_any_element()
            }
            Screen::Members => {
                screens::MembersScreen::render("screen-members", &self.state, &theme, cx)
                    .into_any_element()
            }
            Screen::Settings => {
                screens::SettingsScreen::render("screen-settings", &self.state, &theme, cx)
                    .into_any_element()
            }
        };

        let top_safe_area = self.state.safe_area.top_px();

        if matches!(self.state.current_screen, Screen::Auth | Screen::Onboarding) {
            return div()
                .flex()
                .flex_col()
                .w_full()
                .h_full()
                .bg(theme.bg_canvas)
                .font_family(theme.font_family)
                .relative()
                .child(screen_element)
                .children(self.render_toast(&theme, top_safe_area, cx));
        }

        div()
            .flex()
            .flex_row()
            .w_full()
            .h_full()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .relative()
            // Desktop Sidebar Navigation
            .child(components::Sidebar::render(
                "app-sidebar",
                &self.state,
                &theme,
                cx,
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
                    .children(self.render_toast(&theme, top_safe_area, cx)),
            )
            // Realtime Notifications Drawer
            .children(if self.state.show_notifications_drawer {
                Some(components::NotificationsDrawer::render(
                    "app-notifications-drawer",
                    &self.state,
                    &theme,
                    cx,
                ))
            } else {
                None
            })
            // Quick Entry Modal Dialog Overlay
            .children(if self.state.show_quick_entry {
                let wallets = self.state.active_wallets();
                let is_family = self.state.active_scope.is_family();
                Some(
                    div()
                        .absolute()
                        .inset_0()
                        .bg(hsla(0.0, 0.0, 0.0, 0.5))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(QuickEntryDialog::render(
                            "app-quick-entry-dialog",
                            &self.quick_entry_form,
                            &self.quick_entry_inputs,
                            wallets,
                            is_family,
                            self.state.is_busy,
                            self.state.locale,
                            &theme,
                            cx,
                        )),
                )
            } else {
                None
            })
            // Entity Creation / Update Modal Overlay
            .children(self.state.active_modal.as_ref().map(|modal| {
                div()
                    .absolute()
                    .inset_0()
                    .bg(hsla(0.0, 0.0, 0.0, 0.5))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(EntityModalDialog::render(
                        "app-entity-modal-dialog",
                        modal,
                        &self.state,
                        &self.entity_modal_inputs,
                        &theme,
                        cx,
                    ))
            }))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_realtime_listener(
    ws_url: String,
    token: String,
) -> (
    RealtimeHandle,
    tokio::sync::mpsc::UnboundedReceiver<RealtimeEvent>,
) {
    RealtimeClient::connect(ws_url, token)
}
