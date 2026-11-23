use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use umuryango_core_domain::ledger::LedgerValidator;
use umuryango_core_domain::models::{
    FamilyMember, FamilyRole, FuturePlan, Loan, Project, Transaction, TransactionType, Wallet,
};
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_core_domain::DomainError;
use umuryango_i18n::{terms, Locale, Translator};
use umuryango_storage_local::{
    CacheRepository, LocalDatabase, SyncMutation, SyncQueueRepository, SyncStatus,
};
use uuid::Uuid;

use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Screen {
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
            Screen::Dashboard => Translator::t(locale, terms::NAV_DASHBOARD, None),
            Screen::Transactions => Translator::t(locale, terms::NAV_TRANSACTIONS, None),
            Screen::Reports => Translator::t(locale, terms::NAV_REPORTS, None),
            Screen::Projects => Translator::t(locale, terms::NAV_PROJECTS, None),
            Screen::Loans => Translator::t(locale, terms::NAV_LOANS, None),
            Screen::Plans => Translator::t(locale, terms::NAV_PLANS, None),
            Screen::Members => Translator::t(locale, terms::NAV_MEMBERS, None),
            Screen::Settings => {
                if locale == Locale::Rw {
                    "Igenamiterere".to_string()
                } else {
                    "Settings".to_string()
                }
            }
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
}

impl Default for AppState {
    fn default() -> Self {
        let user_id = Uuid::new_v4();
        let family_id = Uuid::new_v4();
        let theme_config = ThemeConfig::default();
        let theme_mode = ThemeMode::Light;
        let theme = SemanticTheme::from_config(&theme_config, theme_mode);

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
            current_screen: Screen::Dashboard,
            toast_message: None,
            show_quick_entry: false,

            show_notifications_drawer: false,
            notifications: Vec::new(),
            unread_notifications_count: 0,
        }
    }
}

impl AppState {
    pub fn new(user_id: Uuid, user_name: String) -> Self {
        let mut state = Self::default();
        state.user_id = user_id;
        state.user_name = user_name;
        state
    }

    /// Load cached state and theme preferences from the local offline SQLite database
    pub fn load_from_db(&mut self, db: &LocalDatabase) {
        let repo = CacheRepository::new(db);
        let queue = SyncQueueRepository::new(db);

        // 1. Theme prefs
        if let Ok(Some(prefs)) = repo.get_theme_prefs(self.user_id) {
            let mode = if prefs.theme_mode.eq_ignore_ascii_case("dark") {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
            self.theme_mode = mode;
            self.theme_config = ThemeConfig::from(&prefs);
            self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);
        }

        // 2. Wallets & Transactions
        if let Ok(wallets) = repo.list_wallets(&self.active_scope) {
            self.wallets = wallets;
        }
        if let Ok(txs) = repo.list_transactions(&self.active_scope, 100) {
            self.transactions = txs;
        }

        // 3. Projects, Loans, Plans
        if let LedgerScope::Family { family_id } = self.active_scope {
            if let Ok(projects) = repo.list_projects(family_id) {
                self.projects = projects;
            }
        }
        if let Ok(loans) = repo.list_loans(&self.active_scope) {
            self.loans = loans;
        }
        if let Ok(plans) = repo.list_future_plans(&self.active_scope) {
            self.future_plans = plans;
        }

        // 4. Sync queue status
        if let Ok(status) = queue.sync_status(true) {
            self.sync_status = status;
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

        if let Some(database) = db {
            self.load_from_db(database);
        }
    }

    pub fn switch_locale(&mut self, locale: Locale) {
        self.locale = locale;
    }

    pub fn toggle_theme(&mut self, db: Option<&LocalDatabase>) {
        self.theme_mode = self.theme_mode.toggle();
        self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);

        if let Some(database) = db {
            let repo = CacheRepository::new(database);
            let prefs = self.theme_config.to_prefs(self.theme_mode);
            let _ = repo.save_theme_prefs(self.user_id, &prefs);
        }
    }

    pub fn set_theme_config(&mut self, config: ThemeConfig, db: Option<&LocalDatabase>) {
        self.theme_config = config;
        self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);

        if let Some(database) = db {
            let repo = CacheRepository::new(database);
            let prefs = self.theme_config.to_prefs(self.theme_mode);
            let _ = repo.save_theme_prefs(self.user_id, &prefs);
        }
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

    /// Handles an incoming RealtimeEvent from the WebSocket connection:
    /// - Updates matching wallet balances locally and in SQLite cache.
    /// - Formats localized notification string in user's active locale.
    /// - Adds unread notification item to drawer list.
    /// - Triggers instant toast message.
    pub fn apply_realtime_event(&mut self, event: RealtimeEvent, db: Option<&LocalDatabase>) {
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
                if let Some(database) = db {
                    let repo = CacheRepository::new(database);
                    let _ = repo.update_wallet_balance(*wallet_id, *new_wallet_balance);
                }
            }
            _ => {}
        }

        let formatted = event.format_notification(self.locale);
        let item = NotificationItem {
            id: Uuid::new_v4(),
            event,
            message: formatted.clone(),
            timestamp: Utc::now(),
            is_read: false,
        };

        self.notifications.insert(0, item);
        self.unread_notifications_count += 1;
        self.set_toast(formatted, false);
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
        assert!(state.notifications[0]
            .message
            .contains("Alice yongeye 50000 RWF"));
        assert_eq!(
            state.toast_message,
            Some((state.notifications[0].message.clone(), false))
        );

        // Mark as read
        state.mark_all_notifications_as_read();
        assert_eq!(state.unread_notifications_count, 0);
        assert!(state.notifications[0].is_read);
    }
}
