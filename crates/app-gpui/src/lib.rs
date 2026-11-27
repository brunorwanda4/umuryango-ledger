mod actions_auth;
mod actions_ledger;
pub mod api_bridge;
pub mod assets;
pub mod common;
pub mod components;
pub mod db_facade;
pub mod hooks;
pub mod mobile;
mod navigation_persistence;
pub mod safe_area;
pub mod screens;
pub mod state;
pub mod theme;
pub mod updater;
pub mod web;
pub use web::mount_gpui_app;

use std::sync::{Arc, Mutex};

use api_bridge::ApiBridge;
use common::{
    TOAST_DURATION, Toast, ToastClose, ToastContent, ToastIcon, ToastPortal, ToastTitle, ToastType,
    ToastViewport,
};
use components::entity_modal_inputs::EntityModalInputs;
use components::entity_modals::EntityModalDialog;
use components::navbar::NavbarInputs;
use components::quick_entry::{QuickEntryDialog, QuickEntryForm};
use components::quick_entry_inputs::QuickEntryInputs;
use gpui_kit::{
    Context, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement, Render,
    Styled, Subscription, Task, Window, div, px,
};
pub use safe_area::SafeAreaInsets;
use screens::auth_inputs::AuthInputs;
use screens::onboarding_inputs::OnboardingInputs;
use state::{AppState, OnboardingStep, Screen};
use theme::ThemeMode;
#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::{RealtimeClient, RealtimeHandle};
#[cfg(not(target_arch = "wasm32"))]
use umuryango_core_domain::realtime::RealtimeEvent;

/// Official 256x256 embedded application icon
pub const APP_ICON_256_PNG: &[u8] = include_bytes!("../assets/icons/icon-256.png");

/// Official 256x256 embedded primary brand logo
pub const APP_LOGO_256_PNG: &[u8] = include_bytes!("../assets/icons/logo-256.png");

pub struct RootView {
    pub state: AppState,
    pub db: Option<Arc<Mutex<crate::db_facade::LocalDatabase>>>,
    pub api: ApiBridge,
    pub quick_entry_form: QuickEntryForm,
    pub auth_inputs: AuthInputs,
    pub onboarding_inputs: OnboardingInputs,
    pub navbar_inputs: NavbarInputs,
    pub quick_entry_inputs: QuickEntryInputs,
    pub entity_modal_inputs: EntityModalInputs,
    protected_focus: FocusHandle,
    toast_scheduled_id: u64,
    _toast_timer_task: Option<Task<()>>,
    _viewport_sub: Subscription,
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
        let restored_token = db.as_ref().and_then(|d| d.lock().ok()).and_then(|locked| {
            let token = state.restore_session_from_db(&locked);
            state.restore_navigation_from_db(&locked);
            token
        });

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
        let navbar_inputs = NavbarInputs::new(locale, window, cx);
        let quick_entry_inputs = QuickEntryInputs::new(window, cx);
        let entity_modal_inputs = EntityModalInputs::new(window, cx);
        let protected_focus = cx.focus_handle();

        if hooks::use_mobile::is_mobile(window) {
            state.sidebar_open = false;
        }
        let mut last_bucket = (
            (hooks::use_mobile::viewport_width(window) as i32) / 16,
            (hooks::use_mobile::viewport_height(window) as i32) / 16,
        );
        let mut last_mobile = hooks::use_mobile::is_mobile(window);
        let viewport_sub =
            hooks::use_mobile::observe_viewport(window, cx, move |this, metrics, _window, cx| {
                let next_bucket = (
                    (f32::from(metrics.size.width) as i32) / 16,
                    (f32::from(metrics.size.height) as i32) / 16,
                );
                let now_mobile = metrics.is_mobile();
                let mut changed = false;

                if this.state.safe_area != metrics.safe_area {
                    this.state.update_safe_area(metrics.safe_area);
                    changed = true;
                }
                if now_mobile != last_mobile {
                    last_mobile = now_mobile;
                    if now_mobile && this.state.sidebar_open {
                        this.state.sidebar_open = false;
                    }
                    changed = true;
                }
                if next_bucket != last_bucket {
                    last_bucket = next_bucket;
                    changed = true;
                }
                if changed {
                    cx.notify();
                }
            });

        #[allow(unused_mut)]
        let mut view = Self {
            state,
            db,
            api,
            quick_entry_form: form,
            auth_inputs,
            onboarding_inputs,
            navbar_inputs,
            quick_entry_inputs,
            entity_modal_inputs,
            protected_focus,
            toast_scheduled_id: 0,
            _toast_timer_task: None,
            _viewport_sub: viewport_sub,
        };

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(token) = restored_token {
            view.verify_restored_session(token, cx);
        }

        view
    }

    pub fn select_screen(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.state.select_screen(screen);
        self.persist_navigation();
        cx.notify();
    }

    pub fn toggle_auth_mode(&mut self, cx: &mut Context<Self>) {
        if !self.state.is_busy {
            self.state.auth_mode_register = !self.state.auth_mode_register;
            self.persist_navigation();
            cx.notify();
        }
    }

    pub fn set_onboarding_step(
        &mut self,
        step: OnboardingStep,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.state.is_busy {
            return;
        }
        self.state.onboarding_step = step;
        self.persist_navigation();
        if step == OnboardingStep::JoinFamily {
            self.onboarding_inputs.focus_join_code(window, cx);
        }
        cx.notify();
    }

    pub(crate) fn persist_navigation(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(ref db) = self.db
            && let Ok(locked) = db.lock()
        {
            self.state.save_navigation_to_db(&locked);
        }
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

    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.state.sidebar_open = !self.state.sidebar_open;
        cx.notify();
    }

    pub fn set_families_menu_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.state.families_menu_open != open {
            self.state.families_menu_open = open;
            if open {
                self.state.user_menu_open = false;
            }
            cx.notify();
        }
    }

    pub fn open_command_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.state.command_query.clear();
        self.state.show_command_palette = true;
        self.state.families_menu_open = false;
        self.state.user_menu_open = false;
        self.navbar_inputs
            .reset_and_focus(self.state.locale, window, cx);
        cx.notify();
    }

    pub fn set_command_palette_open(
        &mut self,
        open: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if open {
            self.open_command_palette(window, cx);
        } else if self.state.show_command_palette {
            self.state.show_command_palette = false;
            cx.notify();
        }
    }

    pub fn toggle_notifications(&mut self, cx: &mut Context<Self>) {
        self.state.toggle_notifications_drawer();
        cx.notify();
    }

    pub fn toggle_money_group(&mut self, cx: &mut Context<Self>) {
        self.state.money_expanded = !self.state.money_expanded;
        cx.notify();
    }

    pub fn set_user_menu_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.state.user_menu_open != open {
            self.state.user_menu_open = open;
            if open {
                self.state.families_menu_open = false;
            }
            cx.notify();
        }
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
        self._toast_timer_task = None;
        cx.notify();
    }

    fn sync_toast_timer(&mut self, cx: &mut Context<Self>) {
        if self.state.toast_message.is_none() {
            self._toast_timer_task = None;
            return;
        }
        let target_id = self.state.toast_id;
        if self.toast_scheduled_id == target_id {
            return;
        }
        self.toast_scheduled_id = target_id;
        self._toast_timer_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(TOAST_DURATION).await;
            let _ = this.update(cx, |this, cx| {
                if this.state.toast_id == target_id && this.state.toast_message.is_some() {
                    this.state.clear_toast();
                    cx.notify();
                }
            });
        }));
    }

    pub fn copy_to_clipboard(&mut self, text: String, feedback: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(gpui_kit::ClipboardItem::new_string(text));
        self.state.set_toast(feedback, false);
        cx.notify();
    }

    fn render_toast(
        &self,
        theme: &theme::SemanticTheme,
        cx: &Context<Self>,
    ) -> Option<impl IntoElement> {
        let (msg, is_error) = self.state.toast_message.as_ref()?;
        let border = if *is_error {
            theme.expense_color
        } else {
            theme.income_color
        };

        Some(
            ToastPortal::new().priority(20).child(
                ToastViewport::new().safe_area(self.state.safe_area).child(
                    Toast::new("app-toast-container")
                        .border_color(border)
                        .theme(theme)
                        .child(
                            ToastContent::new()
                                .p(px(12.0))
                                .child(
                                    ToastIcon::new(if *is_error {
                                        ToastType::Error
                                    } else {
                                        ToastType::Success
                                    })
                                    .theme(theme),
                                )
                                .child(
                                    ToastTitle::new(msg.clone())
                                        .min_w_0()
                                        .flex_1()
                                        .text_xs()
                                        .theme(theme),
                                )
                                .child(ToastClose::new("app-toast-dismiss").theme(theme).on_click(
                                    cx.listener(|this, _, _, cx| {
                                        this.dismiss_toast(cx);
                                    }),
                                )),
                        ),
                ),
            ),
        )
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_toast_timer(cx);
        let theme = self.state.theme.clone();
        let screen_element = match self.state.current_screen {
            Screen::Auth => screens::AuthScreen::render(
                "screen-auth",
                &self.state,
                &self.auth_inputs,
                &theme,
                window,
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
            Screen::Dashboard => screens::DashboardScreen::render(
                "screen-dashboard",
                &self.state,
                &theme,
                window,
                cx,
            )
            .into_any_element(),
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

        if matches!(self.state.current_screen, Screen::Auth | Screen::Onboarding) {
            return div()
                .id("public-app-shell")
                .flex()
                .flex_col()
                .w_full()
                .h_full()
                .bg(theme.bg_canvas)
                .font_family(theme.font_family)
                .relative()
                .child(screen_element)
                .children(self.render_toast(&theme, cx));
        }

        div()
            .id("protected-app-shell")
            .track_focus(&self.protected_focus)
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, window, cx| {
                let mods = &ev.keystroke.modifiers;
                if mods.platform || mods.control {
                    if ev.keystroke.key.eq_ignore_ascii_case("k") {
                        let next = !this.state.show_command_palette;
                        this.set_command_palette_open(next, window, cx);
                    } else if ev.keystroke.key.eq_ignore_ascii_case("b") {
                        this.toggle_sidebar(cx);
                    }
                }
            }))
            .flex()
            .flex_row()
            .w_full()
            .h_full()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .relative()
            // Desktop Sidebar Navigation (collapsible via navbar trigger or Cmd/Ctrl+B)
            .children(
                self.state
                    .sidebar_open
                    .then(|| components::Sidebar::render("app-sidebar", &self.state, &theme, cx)),
            )
            // Protected Content Column: Top Workspace Navbar + Update Banner + Active Screen
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .relative()
                    .child(components::Navbar::render(
                        "app-navbar",
                        &self.state,
                        &theme,
                        cx,
                    ))
                    .children(updater::update_banner::UpdateBanner::render(
                        "app-update-banner",
                        &self.state.updater.status,
                        self.state.locale,
                        &theme,
                    ))
                    .child(
                        div()
                            .flex_1()
                            .w_full()
                            .min_h_0()
                            .overflow_hidden()
                            .child(screen_element),
                    ),
            )
            // Global Search & Command Palette (Cmd/Ctrl+K)
            .child(components::Navbar::render_command_dialog(
                "app-command-dialog",
                &self.state,
                &self.navbar_inputs,
                &theme,
                cx,
            ))
            // Realtime Notifications Drawer (SheetOverlay + SheetContent)
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
            // Quick Entry Modal Dialog Overlay (DialogOverlay + SheetContent)
            .children(if self.state.show_quick_entry {
                let wallets = self.state.active_wallets();
                let is_family = self.state.active_scope.is_family();
                Some(QuickEntryDialog::render(
                    "app-quick-entry-dialog",
                    &self.quick_entry_form,
                    &self.quick_entry_inputs,
                    wallets,
                    is_family,
                    self.state.is_busy,
                    self.state.locale,
                    &theme,
                    cx,
                ))
            } else {
                None
            })
            // Entity Creation / Update Modal Overlay (DialogOverlay + DialogContent)
            .children(self.state.active_modal.as_ref().map(|modal| {
                EntityModalDialog::render(
                    "app-entity-modal-dialog",
                    modal,
                    &self.state,
                    &self.entity_modal_inputs,
                    &theme,
                    cx,
                )
            }))
            // Floating Responsive Toast (Top on small screens, Bottom-Right on large screens)
            .children(self.render_toast(&theme, cx))
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
