mod components;
mod mobile;
mod safe_area;
mod screens;
mod state;
mod theme;
pub mod updater;

use std::sync::{Arc, Mutex};
use uuid::Uuid;

use components::icon::{AppIcon, IconSize, IconTone, app_icon};
use components::quick_entry::{QuickEntryDialog, QuickEntryForm, QuickEntryMode};
use gpui_kit::{
    AppContext, Context, InteractiveElement, StatefulInteractiveElement, IntoElement, ParentElement, Render, Styled, Window, div, hsla, px,
};
pub use safe_area::SafeAreaInsets;
use state::{AppState, Screen};
use theme::{SemanticTheme, ThemeMode, archivo_font_bytes};
use umuryango_api_client::{RealtimeClient, RealtimeHandle};
use umuryango_core_domain::models::FamilyRole;
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;
use umuryango_storage_local::{CacheRepository, LocalDatabase};

/// Official 256x256 embedded emblem icon
pub const APP_ICON_256_PNG: &[u8] = include_bytes!("../assets/icons/icon-256.png");

pub struct RootView {
    pub state: AppState,
    pub db: Option<Arc<Mutex<LocalDatabase>>>,
    pub family_id: Uuid,
    pub quick_entry_form: QuickEntryForm,
}

impl RootView {
    pub fn new(state: AppState, db: Option<Arc<Mutex<LocalDatabase>>>) -> Self {
        let family_id = match state.active_scope {
            LedgerScope::Family { family_id } => family_id,
            LedgerScope::Personal { .. } => Uuid::new_v4(),
        };
        let mut form = QuickEntryForm::default();
        form.source_wallet_id = state.active_wallets().first().map(|w| w.id);

        Self {
            state,
            db,
            family_id,
            quick_entry_form: form,
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
            self.state.theme_mode = target_mode;
            self.state.theme =
                SemanticTheme::from_config(&self.state.theme_config, self.state.theme_mode);
            crate::safe_area::platform::set_system_bars_contrast(
                self.state.theme_mode != ThemeMode::Dark,
            );
            if let Some(database) = db_lock.as_deref() {
                let repo = CacheRepository::new(database);
                let prefs = self.state.theme_config.to_prefs(self.state.theme_mode);
                let _ = repo.save_theme_prefs(self.state.user_id, &prefs);
            }
            cx.notify();
        }
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
                        let msg = if self.state.locale == Locale::Rw {
                            "Igikorwa cyanditswe neza!".to_string()
                        } else {
                            "Transaction recorded successfully!".to_string()
                        };
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
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.theme.clone();
        let screen_element = match self.state.current_screen {
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
                            &wallets,
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

    let db_arc = Arc::new(Mutex::new(db));

    println!(
        "Application ready. Active scope: {:?}, Locale: {:?}",
        state.active_scope, state.locale
    );

    let app = gpui_kit::application();
    app.run(move |cx| {
        gpui_kit::init(cx);

        if let Err(e) = cx.text_system().add_fonts(archivo_font_bytes()) {
            eprintln!("Warning: Failed to load bundled Archivo fonts: {:?}", e);
        }

        let mut options = gpui_kit::WindowOptions::default();
        options.titlebar = Some(gpui_kit::TitlebarOptions {
            title: Some("Umuryango Ledger".into()),
            ..Default::default()
        });
        let db_clone = Arc::clone(&db_arc);
        let _ = cx.open_window(options, |_window, cx| {
            cx.new(|_| RootView::new(state, Some(db_clone)))
        });
    });
}
