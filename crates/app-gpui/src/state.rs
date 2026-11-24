use crate::db_facade::LocalDatabase;
#[cfg(not(target_arch = "wasm32"))]
use crate::db_facade::{CacheRepository, SyncMutation, SyncQueueRepository};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use umuryango_core_domain::DomainError;
use umuryango_core_domain::SyncStatus;
use umuryango_core_domain::ledger::LedgerValidator;
use umuryango_core_domain::models::{
    FamilyMember, FamilyRole, FuturePlan, Loan, PendingFamilyInvite, Project, Transaction,
    TransactionType, Wallet, WalletChannel,
};
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::safe_area::SafeAreaInsets;
use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};
use crate::updater::AppUpdater;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
    Auth,
    Onboarding,
    Dashboard,
    Transactions,
    Reports,
    Projects,
    Loans,
    Plans,
    Members,
    Settings,
}

impl Screen {
    pub fn label(&self, locale: Locale) -> String {
        match self {
            Screen::Auth => Translator::t(locale, terms::LOGIN_TITLE, None),
            Screen::Onboarding => Translator::t(locale, terms::ONBOARDING_TITLE, None),
            Screen::Dashboard => Translator::t(locale, terms::NAV_DASHBOARD, None),
            Screen::Transactions => Translator::t(locale, terms::NAV_TRANSACTIONS, None),
            Screen::Reports => Translator::t(locale, terms::NAV_REPORTS, None),
            Screen::Projects => Translator::t(locale, terms::NAV_PROJECTS, None),
            Screen::Loans => Translator::t(locale, terms::NAV_LOANS, None),
            Screen::Plans => Translator::t(locale, terms::NAV_PLANS, None),
            Screen::Members => Translator::t(locale, terms::NAV_MEMBERS, None),
            Screen::Settings => umuryango_i18n::t(locale, "settings.title"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationItem {
    pub id: Uuid,
    pub event: RealtimeEvent,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub is_read: bool,
}

pub struct AppState {
    pub user_id: Uuid,
    pub user_name: String,
    pub active_scope: LedgerScope,
    pub active_family_name: Option<String>,
    pub active_role: FamilyRole,
    pub locale: Locale,
    pub theme_mode: ThemeMode,
    pub theme_config: ThemeConfig,
    pub theme: SemanticTheme,

    pub wallets: Vec<Wallet>,
    pub transactions: Vec<Transaction>,
    pub projects: Vec<Project>,
    pub loans: Vec<Loan>,
    pub future_plans: Vec<FuturePlan>,
    pub members: Vec<FamilyMember>,

    pub sync_status: SyncStatus,
    pub current_screen: Screen,
    pub toast_message: Option<(String, bool)>, // (message, is_error)
    pub show_quick_entry: bool,

    // Realtime Notifications Drawer
    pub show_notifications_drawer: bool,
    pub notifications: Vec<NotificationItem>,
    pub unread_notifications_count: usize,

    // Authentication & Onboarding
    pub is_authenticated: bool,
    pub pending_invites: Vec<PendingFamilyInvite>,
    pub auth_mode_register: bool,
    pub auth_phone: String,
    pub auth_password: String,
    pub auth_confirm_password: String,
    pub auth_full_name: String,
    pub auth_gender: String,
    pub onboarding_family_name: String,
    pub onboarding_member_phones: Vec<String>,
    pub onboarding_join_code: String,

    // Safe Area Insets (status bar, navigation bar, cutouts, IME)
    pub safe_area: SafeAreaInsets,

    // Automatic Self-Updating and OTA Engine
    pub updater: AppUpdater,
}

impl Default for AppState {
    fn default() -> Self {
        let user_id = Uuid::new_v4();
        let family_id = Uuid::new_v4();
        let theme_config = ThemeConfig::default();
        let theme_mode = ThemeMode::Light;
        let theme = SemanticTheme::from_config(&theme_config, theme_mode);
        let safe_area = crate::safe_area::platform::query_system_window_insets();

        Self {
            user_id,
            user_name: "Bruno Hapiman".to_string(),
            active_scope: LedgerScope::Family { family_id },
            active_family_name: Some("Umuryango w'Abahire".to_string()),
            active_role: FamilyRole::Head,
            locale: Locale::Rw,
            theme_mode,
            theme_config,
            theme,

            wallets: Vec::new(),
            transactions: Vec::new(),
            projects: Vec::new(),
            loans: Vec::new(),
            future_plans: Vec::new(),
            members: Vec::new(),

            sync_status: SyncStatus::Synced,
            current_screen: Screen::Auth,
            toast_message: None,
            show_quick_entry: false,

            show_notifications_drawer: false,
            notifications: Vec::new(),
            unread_notifications_count: 0,

            is_authenticated: false,
            pending_invites: Vec::new(),
            auth_mode_register: false,
            auth_phone: String::new(),
            auth_password: String::new(),
            auth_confirm_password: String::new(),
            auth_full_name: String::new(),
            auth_gender: "MALE".to_string(),
            onboarding_family_name: String::new(),
            onboarding_member_phones: vec![String::new()],
            onboarding_join_code: String::new(),

            safe_area,
            updater: AppUpdater::new(env!("CARGO_PKG_VERSION").to_string()),
        }
    }
}

impl AppState {
    pub fn new(user_id: Uuid, user_name: String) -> Self {
        Self {
            user_id,
            user_name,
            ..Default::default()
        }
    }

    pub fn update_safe_area(&mut self, insets: SafeAreaInsets) {
        self.safe_area = insets;
    }

    pub fn update_ime_height(&mut self, height: f32) {
        self.safe_area.ime_height = height;
    }

    /// Reloads cached entities matching active scope from the local database
    #[cfg(target_arch = "wasm32")]
    pub fn load_from_db(&mut self, _db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_db(&mut self, db: &LocalDatabase) {
        let repo = CacheRepository::new(db);
        let queue = SyncQueueRepository::new(db);

        // Load theme preferences
        if let Ok(Some(prefs)) = repo.get_theme_prefs(self.user_id) {
            self.theme_config = ThemeConfig::from_prefs(&prefs);
            self.theme_mode = if prefs.theme_mode == "dark" {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
            self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);
            crate::safe_area::platform::set_system_bars_contrast(
                self.theme_mode != ThemeMode::Dark,
            );
        }

        // Load sync status
        if let Ok(st) = queue.sync_status(true) {
            self.sync_status = st;
        }

        // Load cached wallets & transactions matching active scope
        match self.active_scope {
            LedgerScope::Personal { user_id } => {
                if let Ok(w) = repo.list_wallets(&self.active_scope) {
                    if w.is_empty() {
                        let seed_wallets = vec![
                            Wallet {
                                id: Uuid::new_v4(),
                                scope: self.active_scope,
                                channel: WalletChannel::MobileMoney,
                                name: "Personal MTN MoMo".to_string(),
                                account_number: Some("+250788112233".to_string()),
                                currency: "RWF".to_string(),
                                balance: Decimal::from(65_000),
                                custodian_id: Some(user_id),
                                is_active: true,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            },
                            Wallet {
                                id: Uuid::new_v4(),
                                scope: self.active_scope,
                                channel: WalletChannel::CashInHand,
                                name: "Personal Cash".to_string(),
                                account_number: None,
                                currency: "RWF".to_string(),
                                balance: Decimal::from(20_000),
                                custodian_id: Some(user_id),
                                is_active: true,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            },
                        ];
                        for sw in &seed_wallets {
                            let _ = repo.upsert_wallet(sw);
                        }
                        self.wallets = seed_wallets;
                    } else {
                        self.wallets = w;
                    }
                }
                if let Ok(txs) = repo.list_transactions(&self.active_scope, 100) {
                    self.transactions = txs;
                }
            }
            LedgerScope::Family { family_id } => {
                if let Ok(w) = repo.list_wallets(&self.active_scope) {
                    if w.is_empty() {
                        let seed_wallets = vec![
                            Wallet {
                                id: Uuid::new_v4(),
                                scope: self.active_scope,
                                channel: WalletChannel::MobileMoney,
                                name: "MTN MoMo Family Custody".to_string(),
                                account_number: Some("+250788123456".to_string()),
                                currency: "RWF".to_string(),
                                balance: Decimal::from(350_000),
                                custodian_id: Some(self.user_id),
                                is_active: true,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            },
                            Wallet {
                                id: Uuid::new_v4(),
                                scope: self.active_scope,
                                channel: WalletChannel::Bank,
                                name: "BK Family Operating Account".to_string(),
                                account_number: Some("00045-0123456-78".to_string()),
                                currency: "RWF".to_string(),
                                balance: Decimal::from(1_200_000),
                                custodian_id: Some(self.user_id),
                                is_active: true,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            },
                            Wallet {
                                id: Uuid::new_v4(),
                                scope: self.active_scope,
                                channel: WalletChannel::CashInHand,
                                name: "Cash in Hand / Ntoki".to_string(),
                                account_number: None,
                                currency: "RWF".to_string(),
                                balance: Decimal::from(80_000),
                                custodian_id: Some(self.user_id),
                                is_active: true,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            },
                        ];
                        for sw in &seed_wallets {
                            let _ = repo.upsert_wallet(sw);
                        }
                        self.wallets = seed_wallets;
                    } else {
                        self.wallets = w;
                    }
                }
                if let Ok(txs) = repo.list_transactions(&self.active_scope, 100) {
                    self.transactions = txs;
                }
                if let Ok(projects) = repo.list_projects(family_id) {
                    self.projects = projects;
                }
                if let Ok(loans) = repo.list_loans(&self.active_scope) {
                    self.loans = loans;
                }
                if let Ok(plans) = repo.list_future_plans(&self.active_scope) {
                    self.future_plans = plans;
                }
            }
        }
    }

    pub fn switch_scope(
        &mut self,
        new_scope: LedgerScope,
        family_name: Option<String>,
        role: FamilyRole,
        db: Option<&LocalDatabase>,
    ) {
        self.active_scope = new_scope;
        self.active_family_name = family_name;
        self.active_role = role;

        #[cfg(not(target_arch = "wasm32"))]
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db {
            self.load_from_db(database);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;
        #[cfg(target_arch = "wasm32")]
        let _ = db;
    }

    pub fn select_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn active_wallets(&self) -> &[Wallet] {
        &self.wallets
    }

    pub fn switch_locale(&mut self, locale: Locale) {
        self.locale = locale;
    }

    pub fn toggle_theme(&mut self, db: Option<&LocalDatabase>) {
        let new_mode = self.theme_mode.toggle();
        self.set_theme_mode(new_mode, db);
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode, db: Option<&LocalDatabase>) {
        self.theme_mode = mode;
        self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);
        crate::safe_area::platform::set_system_bars_contrast(self.theme_mode != ThemeMode::Dark);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db {
            let repo = CacheRepository::new(database);
            let prefs = self.theme_config.to_prefs(self.theme_mode);
            let _ = repo.save_theme_prefs(self.user_id, &prefs);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;
    }

    pub fn set_theme_config(&mut self, config: ThemeConfig, db: Option<&LocalDatabase>) {
        self.theme_config = config;
        self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);
        crate::safe_area::platform::set_system_bars_contrast(self.theme_mode != ThemeMode::Dark);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db {
            let repo = CacheRepository::new(database);
            let prefs = self.theme_config.to_prefs(self.theme_mode);
            let _ = repo.save_theme_prefs(self.user_id, &prefs);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;
    }

    pub fn set_toast(&mut self, message: String, is_error: bool) {
        self.toast_message = Some((message, is_error));
    }

    pub fn clear_toast(&mut self) {
        self.toast_message = None;
    }

    pub fn toggle_notifications_drawer(&mut self) {
        self.show_notifications_drawer = !self.show_notifications_drawer;
    }

    pub fn mark_all_notifications_as_read(&mut self) {
        for n in &mut self.notifications {
            n.is_read = true;
        }
        self.unread_notifications_count = 0;
    }

    /// Process an incoming real-time event received from the WebSocket broadcast hub
    pub fn apply_realtime_event(&mut self, event: RealtimeEvent, db: Option<&LocalDatabase>) {
        let formatted = event.format_notification(self.locale);
        let item = NotificationItem {
            id: Uuid::new_v4(),
            event: event.clone(),
            message: formatted.clone(),
            timestamp: Utc::now(),
            is_read: false,
        };

        // Notify updater if this is an app update announcement
        self.updater.handle_realtime_event(&event);

        // Update in-memory wallet balances directly
        match &event {
            RealtimeEvent::IncomeAdded {
                wallet_id,
                new_wallet_balance,
                ..
            }
            | RealtimeEvent::ExpenseLogged {
                wallet_id,
                new_wallet_balance,
                ..
            } => {
                if let Some(w) = self.wallets.iter_mut().find(|w| w.id == *wallet_id) {
                    w.balance = *new_wallet_balance;
                }
            }
            _ => {}
        }

        self.notifications.insert(0, item);
        self.unread_notifications_count += 1;
        self.set_toast(formatted, false);

        if let Some(database) = db {
            self.load_from_db(database);
        }
    }

    /// Logs a transaction offline: validates invariants via core-domain, applies balance updates,
    /// and persists to SQLite cache and sync queue.
    pub fn log_transaction(
        &mut self,
        tx: Transaction,
        actor_role: FamilyRole,
        db: Option<&LocalDatabase>,
    ) -> Result<(), DomainError> {
        // 1. Validate and apply financial invariants
        LedgerValidator::validate_and_apply_transaction(&tx, actor_role, &mut self.wallets)?;
        // 2. Persist locally if DB provided
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db {
            let repo = CacheRepository::new(database);
            let queue = SyncQueueRepository::new(database);

            let _ = repo.save_transaction(&tx);
            for w in &self.wallets {
                let _ = repo.upsert_wallet(w);
            }

            // Queue for cloud sync
            let mutation = match tx.tx_type {
                TransactionType::Handover => SyncMutation::CreateHandover {
                    idempotency_key: tx.id,
                    family_id: self.active_scope.family_id().unwrap_or_default(),
                    source_wallet_id: tx.source_wallet_id,
                    destination_wallet_id: tx.destination_wallet_id.unwrap_or_default(),
                    amount: tx.amount,
                    notes: tx.description.clone(),
                },
                _ => SyncMutation::CreateTransaction {
                    idempotency_key: tx.id,
                    scope: self.active_scope.as_str().to_string(),
                    family_id: self.active_scope.family_id(),
                    source_wallet_id: tx.source_wallet_id,
                    destination_wallet_id: tx.destination_wallet_id,
                    tx_type: tx.tx_type,
                    amount: tx.amount,
                    category: tx.category.clone(),
                    description: tx.description.clone(),
                    paid_to: tx.paid_to.clone(),
                    receipt_url: None,
                    tx_date: Some(tx.tx_date),
                },
            };

            let _ = queue.enqueue(&mutation);
            if let Ok(st) = queue.sync_status(true) {
                self.sync_status = st;
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;

        self.transactions.insert(0, tx);
        Ok(())
    }

    // ================= KPI COMPUTATIONS =================

    pub fn total_balance(&self) -> Decimal {
        self.wallets.iter().map(|w| w.balance).sum()
    }

    pub fn total_income(&self) -> Decimal {
        self.transactions
            .iter()
            .filter(|t| t.tx_type == TransactionType::Income)
            .map(|t| t.amount)
            .sum()
    }

    pub fn total_expenses(&self) -> Decimal {
        self.transactions
            .iter()
            .filter(|t| t.tx_type == TransactionType::Expense)
            .map(|t| t.amount)
            .sum()
    }

    pub fn active_projects_budget(&self) -> Decimal {
        self.projects.iter().map(|p| p.target_amount).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use umuryango_core_domain::models::WalletChannel;

    #[test]
    fn test_apply_realtime_income_event() {
        let mut state = AppState::default();
        state.locale = Locale::Rw;

        let wallet_id = Uuid::new_v4();
        let family_id = state.active_scope.family_id().unwrap();

        state.wallets.push(Wallet {
            id: wallet_id,
            scope: LedgerScope::Family { family_id },
            channel: WalletChannel::MobileMoney,
            name: "MoMo Main".to_string(),
            account_number: None,
            currency: "RWF".to_string(),
            balance: Decimal::new(20000, 0),
            custodian_id: Some(state.user_id),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        assert_eq!(state.unread_notifications_count, 0);

        let event = RealtimeEvent::IncomeAdded {
            family_id,
            wallet_id,
            wallet_name: "MoMo Main".to_string(),
            amount: Decimal::new(50000, 0),
            currency: "RWF".to_string(),
            added_by_name: "Alice".to_string(),
            source_description: "Salary".to_string(),
            new_wallet_balance: Decimal::new(70000, 0),
            occurred_at: Utc::now(),
        };

        state.apply_realtime_event(event, None);

        // Check wallet balance updated
        assert_eq!(state.wallets[0].balance, Decimal::new(70000, 0));
        assert_eq!(state.unread_notifications_count, 1);
        assert_eq!(state.notifications.len(), 1);
        assert!(
            state.notifications[0]
                .message
                .contains("Alice yongeye 50000 RWF")
        );
        assert_eq!(
            state.toast_message,
            Some((state.notifications[0].message.clone(), false))
        );

        // Mark as read
        state.mark_all_notifications_as_read();
        assert_eq!(state.unread_notifications_count, 0);
        assert!(state.notifications[0].is_read);
    }

    #[test]
    fn test_safe_area_insets_state() {
        let mut state = AppState::default();
        assert_eq!(state.safe_area.top, 0.0);
        let insets = SafeAreaInsets::new(32.0, 48.0, 0.0, 0.0).with_ime(200.0);
        state.update_safe_area(insets);
        assert_eq!(state.safe_area.top, 32.0);
        assert_eq!(state.safe_area.total_bottom(), 200.0);
    }

    #[test]
    fn test_app_update_realtime_handling() {
        let mut state = AppState::default();
        let event = RealtimeEvent::AppUpdateAvailable {
            version: "0.2.0".to_string(),
            platform: "WINDOWS".to_string(),
            is_mandatory: false,
        };
        state.apply_realtime_event(event, None);
        if cfg!(windows) {
            match state.updater.status {
                crate::updater::UpdateStatus::Available { version, .. } => {
                    assert_eq!(version, "0.2.0");
                }
                _ => panic!("Expected update available"),
            }
        }
    }
}
