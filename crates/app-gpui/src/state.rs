use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use umuryango_core_domain::DomainError;
use umuryango_core_domain::SyncStatus;
use umuryango_core_domain::ledger::LedgerValidator;
use umuryango_core_domain::models::{
    FamilyMemberView, FamilyRole, FamilyWithRole, FuturePlan, Loan, LoanDirection,
    PendingFamilyInvite, Project, Transaction, TransactionType, Wallet, WalletChannel,
};
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::api_bridge::{ScopeSnapshot, resolve_api_urls};
use crate::db_facade::LocalDatabase;
#[cfg(not(target_arch = "wasm32"))]
use crate::db_facade::{CacheRepository, SyncMutation, SyncQueueRepository};
use crate::safe_area::SafeAreaInsets;
use crate::screens::dashboard::metrics::DashboardPeriod;
use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};
use crate::updater::AppUpdater;

const SESSION_KEY_TOKEN: &str = "session.token";
const SESSION_KEY_USER_ID: &str = "session.user_id";
const SESSION_KEY_USER_NAME: &str = "session.user_name";
const SESSION_KEY_USER_PHONE: &str = "session.user_phone";
pub(crate) const SESSION_KEY_LOCALE: &str = "session.locale";
const SESSION_KEY_FAMILY_ID: &str = "session.active_family_id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OnboardingStep {
    #[default]
    Choose,
    CreateFamily,
    JoinFamily,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveModal {
    AddWallet,
    AddProject,
    AddLoan,
    RepayLoan {
        loan_id: Uuid,
        counterparty: String,
        remaining: Decimal,
    },
    AddPlan,
    AllocatePlan {
        plan_id: Uuid,
        title: String,
    },
    InviteMember,
}

#[derive(Debug, Clone)]
pub struct EntityFormState {
    pub primary_text: String,
    pub secondary_text: String,
    pub amount_text: String,
    pub wallet_channel: WalletChannel,
    pub loan_direction: LoanDirection,
    pub invite_role: FamilyRole,
}

impl Default for EntityFormState {
    fn default() -> Self {
        Self {
            primary_text: String::new(),
            secondary_text: String::new(),
            amount_text: String::new(),
            wallet_channel: WalletChannel::MobileMoney,
            loan_direction: LoanDirection::Lent,
            invite_role: FamilyRole::Member,
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
    pub user_phone: String,
    pub auth_token: Option<String>,
    pub api_base_url: String,

    pub active_scope: LedgerScope,
    pub active_family_id: Option<Uuid>,
    pub active_family_name: Option<String>,
    pub active_family_invite_code: Option<String>,
    pub active_role: FamilyRole,
    pub families: Vec<FamilyWithRole>,
    pub sidebar_open: bool,
    pub money_expanded: bool,
    pub user_menu_open: bool,
    pub families_menu_open: bool,
    pub show_command_palette: bool,
    pub command_query: String,
    pub dashboard_period: Option<DashboardPeriod>,

    pub locale: Locale,
    pub theme_mode: ThemeMode,
    pub theme_config: ThemeConfig,
    pub theme: SemanticTheme,

    pub wallets: Vec<Wallet>,
    pub wallet_custodian_names: HashMap<Uuid, String>,
    pub transactions: Vec<Transaction>,
    pub projects: Vec<Project>,
    pub loans: Vec<Loan>,
    pub future_plans: Vec<FuturePlan>,
    pub family_members: Vec<FamilyMemberView>,

    pub sync_status: SyncStatus,
    pub is_busy: bool,
    pub current_screen: Screen,
    pub toast_message: Option<(String, bool)>,
    pub toast_id: u64,
    pub show_quick_entry: bool,
    pub active_modal: Option<ActiveModal>,
    pub entity_form: EntityFormState,

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
    pub onboarding_step: OnboardingStep,
    pub onboarding_family_name: String,
    pub onboarding_phone_draft: String,
    pub onboarding_member_phones: Vec<String>,
    pub onboarding_join_code: String,

    // Safe Area Insets (status bar, navigation bar, cutouts, IME)
    pub safe_area: SafeAreaInsets,

    // Automatic Self-Updating and OTA Engine
    pub updater: AppUpdater,
}

impl Default for AppState {
    fn default() -> Self {
        let user_id = Uuid::nil();
        let theme_config = ThemeConfig::default();
        let theme_mode = ThemeMode::Light;
        let theme = SemanticTheme::from_config(&theme_config, theme_mode);
        let safe_area = crate::safe_area::platform::query_system_window_insets();
        let (api_base_url, _) = resolve_api_urls();

        Self {
            user_id,
            user_name: String::new(),
            user_phone: String::new(),
            auth_token: None,
            api_base_url,

            active_scope: LedgerScope::Personal { user_id },
            active_family_id: None,
            active_family_name: None,
            active_family_invite_code: None,
            active_role: FamilyRole::Member,
            families: Vec::new(),
            sidebar_open: true,
            money_expanded: true,
            user_menu_open: false,
            families_menu_open: false,
            show_command_palette: false,
            command_query: String::new(),
            dashboard_period: None,

            locale: Locale::Rw,
            theme_mode,
            theme_config,
            theme,

            wallets: Vec::new(),
            wallet_custodian_names: HashMap::new(),
            transactions: Vec::new(),
            projects: Vec::new(),
            loans: Vec::new(),
            future_plans: Vec::new(),
            family_members: Vec::new(),

            sync_status: SyncStatus::Synced,
            is_busy: false,
            current_screen: Screen::Auth,
            toast_message: None,
            toast_id: 0,
            show_quick_entry: false,
            active_modal: None,
            entity_form: EntityFormState::default(),

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
            onboarding_step: OnboardingStep::Choose,
            onboarding_family_name: String::new(),
            onboarding_phone_draft: String::new(),
            onboarding_member_phones: Vec::new(),
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
            active_scope: LedgerScope::Personal { user_id },
            ..Default::default()
        }
    }

    pub fn update_safe_area(&mut self, insets: SafeAreaInsets) {
        self.safe_area = insets;
    }

    pub fn update_ime_height(&mut self, height: f32) {
        self.safe_area.ime_height = height;
    }

    /// Reloads cached entities matching active scope from SQLite (never seeds fake records).
    #[cfg(target_arch = "wasm32")]
    pub fn load_from_db(&mut self, _db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_db(&mut self, db: &LocalDatabase) {
        let repo = CacheRepository::new(db);
        let queue = SyncQueueRepository::new(db);

        if !self.user_id.is_nil()
            && let Ok(Some(prefs)) = repo.get_theme_prefs(self.user_id)
        {
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

        if let Ok(st) = queue.sync_status(true) {
            self.sync_status = st;
        }

        if !self.is_authenticated {
            return;
        }

        self.wallets = repo.list_wallets(&self.active_scope).unwrap_or_default();
        self.transactions = repo
            .list_transactions(&self.active_scope, 100)
            .unwrap_or_default();
        self.loans = repo.list_loans(&self.active_scope).unwrap_or_default();
        self.future_plans = repo
            .list_future_plans(&self.active_scope)
            .unwrap_or_default();
        self.projects = match self.active_scope {
            LedgerScope::Family { family_id } => repo.list_projects(family_id).unwrap_or_default(),
            LedgerScope::Personal { .. } => Vec::new(),
        };
    }

    /// Persists active authenticated session metadata into local SQLite `app_config`.
    #[cfg(target_arch = "wasm32")]
    pub fn save_session_to_db(&self, _db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_session_to_db(&self, db: &LocalDatabase) {
        if let Some(ref token) = self.auth_token {
            let _ = db.set_config(SESSION_KEY_TOKEN, token);
            let _ = db.set_config(SESSION_KEY_USER_ID, &self.user_id.to_string());
            let _ = db.set_config(SESSION_KEY_USER_NAME, &self.user_name);
            let _ = db.set_config(SESSION_KEY_USER_PHONE, &self.user_phone);
            let _ = db.set_config(SESSION_KEY_LOCALE, self.locale.as_str());
            let fam_val = self
                .active_family_id
                .map(|id| id.to_string())
                .unwrap_or_default();
            let _ = db.set_config(SESSION_KEY_FAMILY_ID, &fam_val);
        }
    }

    /// Restores a previously saved session token & user profile from SQLite `app_config`.
    #[cfg(target_arch = "wasm32")]
    pub fn restore_session_from_db(&mut self, _db: &LocalDatabase) -> Option<String> {
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn restore_session_from_db(&mut self, db: &LocalDatabase) -> Option<String> {
        let token = db.get_config(SESSION_KEY_TOKEN).ok()??;
        if token.trim().is_empty() {
            return None;
        }
        let user_id_str = db.get_config(SESSION_KEY_USER_ID).ok()??;
        let user_id = Uuid::parse_str(&user_id_str).ok()?;
        let user_name = db
            .get_config(SESSION_KEY_USER_NAME)
            .ok()
            .flatten()
            .unwrap_or_default();
        let user_phone = db
            .get_config(SESSION_KEY_USER_PHONE)
            .ok()
            .flatten()
            .unwrap_or_default();
        if let Ok(Some(loc)) = db.get_config(SESSION_KEY_LOCALE) {
            self.locale = match loc.as_str() {
                "en" => Locale::En,
                _ => Locale::Rw,
            };
        }
        let saved_family_id = db
            .get_config(SESSION_KEY_FAMILY_ID)
            .ok()
            .flatten()
            .and_then(|s| Uuid::parse_str(&s).ok());

        self.user_id = user_id;
        self.user_name = user_name;
        self.user_phone = user_phone;
        self.auth_token = Some(token.clone());
        self.is_authenticated = true;
        self.active_family_id = saved_family_id;
        self.active_scope = match saved_family_id {
            Some(family_id) => LedgerScope::Family { family_id },
            None => LedgerScope::Personal { user_id },
        };
        self.current_screen = Screen::Dashboard;
        self.load_from_db(db);
        Some(token)
    }

    /// Clears persisted session keys from SQLite `app_config`.
    #[cfg(target_arch = "wasm32")]
    pub fn clear_session_in_db(_db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear_session_in_db(db: &LocalDatabase) {
        for key in [
            SESSION_KEY_TOKEN,
            SESSION_KEY_USER_ID,
            SESSION_KEY_USER_NAME,
            SESSION_KEY_USER_PHONE,
            SESSION_KEY_FAMILY_ID,
        ] {
            let _ = db.set_config(key, "");
        }
    }

    /// Resets all user-scoped state upon logout or expired session.
    pub fn reset_session(&mut self) {
        self.user_id = Uuid::nil();
        self.user_name.clear();
        self.user_phone.clear();
        self.auth_token = None;
        self.is_authenticated = false;
        self.active_scope = LedgerScope::Personal {
            user_id: Uuid::nil(),
        };
        self.active_family_id = None;
        self.active_family_name = None;
        self.active_family_invite_code = None;
        self.active_role = FamilyRole::Member;
        self.families.clear();
        self.wallets.clear();
        self.wallet_custodian_names.clear();
        self.transactions.clear();
        self.projects.clear();
        self.loans.clear();
        self.future_plans.clear();
        self.family_members.clear();
        self.pending_invites.clear();
        self.notifications.clear();
        self.unread_notifications_count = 0;
        self.show_notifications_drawer = false;
        self.show_quick_entry = false;
        self.active_modal = None;
        self.user_menu_open = false;
        self.families_menu_open = false;
        self.show_command_palette = false;
        self.command_query.clear();
        self.dashboard_period = None;
        self.is_busy = false;
        self.auth_password.clear();
        self.auth_confirm_password.clear();
        self.onboarding_step = OnboardingStep::Choose;
        self.onboarding_family_name.clear();
        self.onboarding_phone_draft.clear();
        self.onboarding_member_phones.clear();
        self.onboarding_join_code.clear();
        self.auth_mode_register = false;
        self.current_screen = Screen::Auth;
    }

    /// Applies an authoritative snapshot fetched from the backend PostgreSQL database.
    pub fn apply_scope_snapshot(&mut self, snap: ScopeSnapshot) {
        self.families = snap.families;
        self.pending_invites = snap.pending_invites;

        if snap.scope == self.active_scope {
            self.wallets = snap.wallets;
            self.wallet_custodian_names = snap.custodian_names;
            self.transactions = snap.transactions;
            self.projects = snap.projects;
            self.loans = snap.loans;
            self.future_plans = snap.future_plans;
            self.family_members = snap.members;
        }

        if let LedgerScope::Family { family_id } = self.active_scope
            && let Some(fam) = self.families.iter().find(|f| f.id == family_id)
        {
            self.active_family_id = Some(fam.id);
            self.active_family_name = Some(fam.name.clone());
            self.active_family_invite_code = fam.invite_code.clone();
            self.active_role = fam.role;
        }

        self.sync_status = if snap.pending_sync_count > 0 {
            SyncStatus::PendingSync(snap.pending_sync_count)
        } else {
            SyncStatus::Synced
        };
    }

    pub fn switch_scope(
        &mut self,
        new_scope: LedgerScope,
        family_name: Option<String>,
        invite_code: Option<String>,
        role: FamilyRole,
        db: Option<&LocalDatabase>,
    ) {
        self.active_scope = new_scope;
        self.active_family_id = new_scope.family_id();
        self.active_family_name = family_name;
        self.active_family_invite_code = invite_code;
        self.active_role = role;
        self.dashboard_period = None;
        self.wallets.clear();
        self.wallet_custodian_names.clear();
        self.transactions.clear();
        self.projects.clear();
        self.loans.clear();
        self.future_plans.clear();
        self.family_members.clear();

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db {
            self.load_from_db(database);
            self.save_session_to_db(database);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;
    }

    pub fn select_screen(&mut self, screen: Screen) {
        if screen == Screen::Onboarding {
            self.onboarding_step = OnboardingStep::Choose;
        }
        self.user_menu_open = false;
        self.families_menu_open = false;
        self.show_command_palette = false;
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
            self.save_theme_mode_to_db(database);
            if !self.user_id.is_nil() {
                let repo = CacheRepository::new(database);
                let prefs = self.theme_config.to_prefs(self.theme_mode);
                let _ = repo.save_theme_prefs(self.user_id, &prefs);
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;
    }

    pub fn set_theme_config(&mut self, config: ThemeConfig, db: Option<&LocalDatabase>) {
        self.theme_config = config;
        self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);
        crate::safe_area::platform::set_system_bars_contrast(self.theme_mode != ThemeMode::Dark);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db
            && !self.user_id.is_nil()
        {
            let repo = CacheRepository::new(database);
            let prefs = self.theme_config.to_prefs(self.theme_mode);
            let _ = repo.save_theme_prefs(self.user_id, &prefs);
        }
        #[cfg(target_arch = "wasm32")]
        let _ = db;
    }

    pub fn set_toast(&mut self, message: String, is_error: bool) {
        self.toast_id = self.toast_id.wrapping_add(1);
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

    /// Resolves the custodian display name for a wallet from real PostgreSQL metadata.
    pub fn custodian_name_for_wallet(&self, wallet: &Wallet) -> Option<String> {
        if let Some(name) = self.wallet_custodian_names.get(&wallet.id) {
            return Some(name.clone());
        }
        if let Some(cid) = wallet.custodian_id {
            return self.actor_name(cid);
        }
        None
    }

    /// Resolves a user ID to a real member name (or current user's name).
    pub fn actor_name(&self, user_id: Uuid) -> Option<String> {
        if user_id == self.user_id && !self.user_name.is_empty() {
            return Some(self.user_name.clone());
        }
        self.family_members
            .iter()
            .find(|m| m.user_id == user_id)
            .map(|m| {
                m.display_name
                    .clone()
                    .unwrap_or_else(|| m.full_name.clone())
            })
    }

    /// Process an incoming real-time event received from the WebSocket broadcast hub.
    pub fn apply_realtime_event(&mut self, event: RealtimeEvent, db: Option<&LocalDatabase>) {
        let formatted = event.format_notification(self.locale);
        let item = NotificationItem {
            id: Uuid::new_v4(),
            event: event.clone(),
            message: formatted.clone(),
            timestamp: Utc::now(),
            is_read: false,
        };

        self.updater.handle_realtime_event(&event);

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
    /// and persists to SQLite cache and sync queue for later cloud replay.
    pub fn log_transaction_offline(
        &mut self,
        tx: Transaction,
        actor_role: FamilyRole,
        db: Option<&LocalDatabase>,
    ) -> Result<(), DomainError> {
        LedgerValidator::validate_and_apply_transaction(&tx, actor_role, &mut self.wallets)?;

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(database) = db {
            let repo = CacheRepository::new(database);
            let queue = SyncQueueRepository::new(database);

            let _ = repo.save_transaction(&tx);
            for w in &self.wallets {
                let _ = repo.upsert_wallet(w);
            }

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
            if let Ok(st) = queue.sync_status(false) {
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

    /// Computes dynamic expense breakdown by category from real transactions.
    /// Returns `(category, amount, percentage)` sorted descending by amount.
    pub fn expense_category_breakdown(&self) -> Vec<(String, Decimal, i32)> {
        let mut totals: HashMap<String, Decimal> = HashMap::new();
        let mut grand_total = Decimal::ZERO;

        for tx in &self.transactions {
            if tx.tx_type == TransactionType::Expense && tx.amount > Decimal::ZERO {
                *totals.entry(tx.category.clone()).or_insert(Decimal::ZERO) += tx.amount;
                grand_total += tx.amount;
            }
        }

        if grand_total <= Decimal::ZERO {
            return Vec::new();
        }

        let mut rows: Vec<(String, Decimal, i32)> = totals
            .into_iter()
            .map(|(cat, amt)| {
                let pct = ((amt / grand_total) * Decimal::from(100))
                    .round()
                    .to_i32()
                    .unwrap_or(0);
                (cat, amt, pct)
            })
            .collect();

        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        rows.truncate(6);
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state_has_no_fake_seed_data() {
        let state = AppState::default();
        assert!(!state.is_authenticated);
        assert!(state.user_name.is_empty());
        assert!(state.active_family_name.is_none());
        assert!(state.wallets.is_empty());
        assert!(state.transactions.is_empty());
        assert!(state.family_members.is_empty());
        assert!(state.sidebar_open);
        assert!(!state.families_menu_open);
        assert!(!state.show_command_palette);
        assert!(state.dashboard_period.is_none());
        assert_eq!(state.current_screen, Screen::Auth);
        assert_eq!(state.onboarding_step, OnboardingStep::Choose);
        assert_eq!(state.toast_id, 0);
    }

    #[test]
    fn test_set_toast_increments_id_and_clear_resets_message() {
        let mut state = AppState::default();
        state.set_toast("First".to_string(), false);
        assert_eq!(state.toast_id, 1);
        assert_eq!(state.toast_message, Some(("First".to_string(), false)));

        state.set_toast("Second".to_string(), true);
        assert_eq!(state.toast_id, 2);
        assert_eq!(state.toast_message, Some(("Second".to_string(), true)));

        state.clear_toast();
        assert_eq!(state.toast_id, 2);
        assert!(state.toast_message.is_none());
    }

    #[test]
    fn test_apply_realtime_income_event() {
        let family_id = Uuid::new_v4();
        let mut state = AppState {
            locale: Locale::Rw,
            user_id: Uuid::new_v4(),
            active_scope: LedgerScope::Family { family_id },
            ..Default::default()
        };

        let wallet_id = Uuid::new_v4();
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

        state.mark_all_notifications_as_read();
        assert_eq!(state.unread_notifications_count, 0);
        assert!(state.notifications[0].is_read);
    }

    #[test]
    fn test_expense_category_breakdown_from_real_transactions() {
        let mut state = AppState::default();
        let uid = Uuid::new_v4();
        let wid = Uuid::new_v4();
        let scope = LedgerScope::Personal { user_id: uid };
        let now = Utc::now();

        state.transactions.push(Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: wid,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::from(7500),
            currency: "RWF".to_string(),
            category: "Food".to_string(),
            description: None,
            authorized_by: uid,
            paid_to: None,
            tx_date: now,
            created_at: now,
        });
        state.transactions.push(Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: wid,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::from(2500),
            currency: "RWF".to_string(),
            category: "Transport".to_string(),
            description: None,
            authorized_by: uid,
            paid_to: None,
            tx_date: now,
            created_at: now,
        });

        let rows = state.expense_category_breakdown();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], ("Food".to_string(), Decimal::from(7500), 75));
        assert_eq!(rows[1], ("Transport".to_string(), Decimal::from(2500), 25));
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
