pub mod common;
pub mod components;
pub mod db_facade;
pub mod mobile;
pub mod safe_area;
pub mod screens;
pub mod state;
pub mod theme;
pub mod updater;
pub mod web;
pub use web::mount_gpui_app;

use std::sync::{Arc, Mutex};
use uuid::Uuid;

use components::icon::{AppIcon, IconSize, IconTone, app_icon};
use components::quick_entry::{QuickEntryDialog, QuickEntryForm, QuickEntryMode};
use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window, div, hsla, px,
};
pub use safe_area::SafeAreaInsets;
use state::{AppState, Screen};
use theme::ThemeMode;
#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::{RealtimeClient, RealtimeHandle};
use umuryango_core_domain::models::FamilyRole;
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;

/// Official 256x256 embedded emblem icon
pub const APP_ICON_256_PNG: &[u8] = include_bytes!("../assets/icons/icon-256.png");

pub struct RootView {
    pub state: AppState,
    pub db: Option<Arc<Mutex<crate::db_facade::LocalDatabase>>>,
    pub family_id: Uuid,
    pub quick_entry_form: QuickEntryForm,
    pub auth_inputs: screens::auth_inputs::AuthInputs,
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
        state: AppState,
        db: Option<Arc<Mutex<crate::db_facade::LocalDatabase>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let family_id = match state.active_scope {
            LedgerScope::Family { family_id } => family_id,
            LedgerScope::Personal { .. } => Uuid::new_v4(),
        };
        state.theme.apply_to_kit(cx);
        let form = QuickEntryForm {
            source_wallet_id: state.active_wallets().first().map(|w| w.id),
            ..Default::default()
        };

        Self {
            state,
            db,
            family_id,
            quick_entry_form: form,
            auth_inputs: screens::auth_inputs::AuthInputs::new(window, cx),
        }
    }

    pub fn select_screen(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.state.select_screen(screen);
        cx.notify();
    }

    pub fn switch_to_family_scope(&mut self, cx: &mut Context<Self>) {
        if !self.state.active_scope.is_family() {
            let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
            let scope = LedgerScope::Family {
                family_id: self.family_id,
            };
            self.state.switch_scope(
                scope,
                Some("Umuryango w'Abahire".to_string()),
                FamilyRole::Head,
                db_lock.as_deref(),
            );
            self.quick_entry_form.source_wallet_id =
                self.state.active_wallets().first().map(|w| w.id);
            cx.notify();
        }
    }

    pub fn switch_to_personal_scope(&mut self, cx: &mut Context<Self>) {
        if self.state.active_scope.is_family() {
            let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
            let scope = LedgerScope::Personal {
                user_id: self.state.user_id,
            };
            self.state
                .switch_scope(scope, None, FamilyRole::Member, db_lock.as_deref());
            self.quick_entry_form.source_wallet_id =
                self.state.active_wallets().first().map(|w| w.id);
            cx.notify();
        }
    }

    pub fn toggle_locale(&mut self, cx: &mut Context<Self>) {
        let new_locale = match self.state.locale {
            Locale::Rw => Locale::En,
            Locale::En => Locale::Rw,
        };
        self.state.switch_locale(new_locale);
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

    pub fn open_quick_entry(&mut self, cx: &mut Context<Self>) {
        if self.quick_entry_form.source_wallet_id.is_none() {
            self.quick_entry_form.source_wallet_id =
                self.state.active_wallets().first().map(|w| w.id);
        }
        self.state.show_quick_entry = true;
        cx.notify();
    }

    pub fn close_quick_entry(&mut self, cx: &mut Context<Self>) {
        self.state.show_quick_entry = false;
        cx.notify();
    }

    pub fn set_quick_entry_mode(&mut self, mode: QuickEntryMode, cx: &mut Context<Self>) {
        self.quick_entry_form.mode = mode;
        cx.notify();
    }

    pub fn submit_quick_entry(&mut self, cx: &mut Context<Self>) {
        let actor_id = self.state.user_id;
        match QuickEntryDialog::build_transaction(
            &self.quick_entry_form,
            &self.state.active_scope,
            actor_id,
        ) {
            Ok(tx) => {
                let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
                let db_ref = db_lock.as_deref();
                match self
                    .state
                    .log_transaction(tx, self.state.active_role, db_ref)
                {
                    Ok(()) => {
                        self.state.show_quick_entry = false;
                        let msg = umuryango_i18n::t(
                            self.state.locale,
                            "transactions.transaction_recorded",
                        );
                        self.state.set_toast(msg, false);
                    }
                    Err(e) => {
                        self.state.set_toast(e.to_string(), true);
                    }
                }
            }
            Err(err) => {
                self.state.set_toast(err, true);
            }
        }
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

    pub fn handle_login_submit(&mut self, cx: &mut Context<Self>) {
        if self.state.auth_phone.trim().is_empty() {
            self.state.auth_phone = "+250788000000".to_string();
        }
        self.state.is_authenticated = true;
        if self.state.active_family_name.is_none() {
            self.state.current_screen = Screen::Onboarding;
        } else {
            self.state.current_screen = Screen::Dashboard;
        }
        let msg = umuryango_i18n::t(self.state.locale, "auth.login_success");
        self.state.set_toast(msg, false);
        cx.notify();
    }

    pub fn handle_register_submit(&mut self, cx: &mut Context<Self>) {
        if self.state.auth_full_name.trim().is_empty() {
            self.state.auth_full_name = "Kwizera Aimable".to_string();
        }
        if self.state.auth_phone.trim().is_empty() {
            self.state.auth_phone = "+250788123456".to_string();
        }
        self.state.user_name = self.state.auth_full_name.clone();
        self.state.is_authenticated = true;
        self.state.current_screen = Screen::Onboarding;
        let msg = umuryango_i18n::t(self.state.locale, "auth.account_created");
        self.state.set_toast(msg, false);
        cx.notify();
    }

    pub fn handle_accept_invite(
        &mut self,
        invite_id: Uuid,
        family_name: String,
        cx: &mut Context<Self>,
    ) {
        let new_family_id = Uuid::new_v4();
        self.state.active_family_name = Some(family_name);
        self.state.active_scope = LedgerScope::Family {
            family_id: new_family_id,
        };
        self.state.pending_invites.retain(|i| i.id != invite_id);
        self.state.current_screen = Screen::Dashboard;
        let msg = umuryango_i18n::Translator::t(
            self.state.locale,
            umuryango_i18n::terms::CONTINUE_TO_DASHBOARD,
            None,
        );
        self.state.set_toast(msg, false);
        cx.notify();
    }

    pub fn handle_create_family_submit(&mut self, cx: &mut Context<Self>) {
        let fam_name = if self.state.onboarding_family_name.trim().is_empty() {
            "Umuryango Wacu".to_string()
        } else {
            self.state.onboarding_family_name.trim().to_string()
        };
        let new_family_id = Uuid::new_v4();
        self.state.active_family_name = Some(fam_name);
        self.state.active_scope = LedgerScope::Family {
            family_id: new_family_id,
        };
        self.state.current_screen = Screen::Dashboard;
        let msg = umuryango_i18n::t_args(
            self.state.locale,
            "onboarding.family_created_code",
            &[("code", "HK0N")],
        );
        self.state.set_toast(msg, false);
        cx.notify();
    }

    pub fn handle_join_family_submit(&mut self, cx: &mut Context<Self>) {
        let code = self.state.onboarding_join_code.trim().to_uppercase();
        let final_code = if code.is_empty() {
            "HK0N".to_string()
        } else {
            code
        };
        let new_family_id = Uuid::new_v4();
        self.state.active_family_name = Some(format!("Family {}", final_code));
        self.state.active_scope = LedgerScope::Family {
            family_id: new_family_id,
        };
        self.state.current_screen = Screen::Dashboard;
        let msg = umuryango_i18n::Translator::t(
            self.state.locale,
            umuryango_i18n::terms::CONTINUE_TO_DASHBOARD,
            None,
        );
        self.state.set_toast(msg, false);
        cx.notify();
    }

    pub fn handle_skip_onboarding(&mut self, cx: &mut Context<Self>) {
        self.state.active_scope = LedgerScope::Personal {
            user_id: self.state.user_id,
        };
        self.state.active_family_name = None;
        self.state.current_screen = Screen::Dashboard;
        cx.notify();
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
            Screen::Onboarding => {
                screens::OnboardingScreen::render("screen-onboarding", &self.state, &theme, cx)
                    .into_any_element()
            }
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
                .children(if let Some((msg, is_error)) = &self.state.toast_message {
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
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .p(px(12.0))
                            .rounded(px(8.0))
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
                                &theme,
                            ))
                            .child(msg.clone()),
                    )
                } else {
                    None
                });
        }

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
                    .children(if let Some((msg, is_error)) = &self.state.toast_message {
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
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .p(px(12.0))
                                .rounded(px(8.0))
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
                                    &theme,
                                ))
                                .child(msg.clone()),
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
                            wallets,
                            is_family,
                            self.state.locale,
                            &theme,
                            cx,
                        )),
                )
            } else {
                None
            })
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
