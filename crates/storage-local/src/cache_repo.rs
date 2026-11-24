use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Result, params};
use rust_decimal::Decimal;
use std::str::FromStr;
use umuryango_core_domain::models::{
    FuturePlan, Loan, LoanDirection, LoanStatus, Project, ProjectStatus, Transaction,
    TransactionType, Wallet, WalletChannel,
};
use umuryango_core_domain::scope::LedgerScope;
use uuid::Uuid;

use crate::db::LocalDatabase;

pub use umuryango_core_domain::UserThemePrefs;

pub struct CacheRepository<'a> {
    db: &'a LocalDatabase,
}

impl<'a> CacheRepository<'a> {
    pub fn new(db: &'a LocalDatabase) -> Self {
        Self { db }
    }

    // ================= WALLETS =================

    pub fn upsert_wallet(&self, wallet: &Wallet) -> Result<()> {
        let (scope_type, fid_str, uid_str) = match wallet.scope {
            LedgerScope::Personal { user_id } => ("personal", None, Some(user_id.to_string())),
            LedgerScope::Family { family_id } => ("family", Some(family_id.to_string()), None),
        };

        let channel_str = match wallet.channel {
            WalletChannel::Bank => "bank",
            WalletChannel::MobileMoney => "momo",
            WalletChannel::CashInHand => "cash",
        };

        self.db.conn().execute(
            r#"
            INSERT INTO wallets (
                id, scope_type, family_id, user_id, channel, name,
                account_number, currency, balance, custodian_id, is_active, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            ON CONFLICT(id) DO UPDATE SET
                scope_type = excluded.scope_type,
                family_id = excluded.family_id,
                user_id = excluded.user_id,
                channel = excluded.channel,
                name = excluded.name,
                account_number = excluded.account_number,
                currency = excluded.currency,
                balance = excluded.balance,
                custodian_id = excluded.custodian_id,
                is_active = excluded.is_active,
                updated_at = excluded.updated_at
            "#,
            params![
                wallet.id.to_string(),
                scope_type,
                fid_str,
                uid_str,
                channel_str,
                wallet.name,
                wallet.account_number,
                wallet.currency,
                wallet.balance.to_string(),
                wallet.custodian_id.map(|u| u.to_string()),
                if wallet.is_active { 1 } else { 0 },
                wallet.created_at.to_rfc3339(),
                wallet.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn update_wallet_balance(&self, wallet_id: Uuid, balance: Decimal) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.conn().execute(
            "UPDATE wallets SET balance = ?1, updated_at = ?2 WHERE id = ?3",
            params![balance.to_string(), now, wallet_id.to_string()],
        )?;
        Ok(())
    }

    pub fn get_wallet(&self, id: Uuid) -> Result<Option<Wallet>> {
        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, scope_type, family_id, user_id, channel, name,
                   account_number, currency, balance, custodian_id, is_active, created_at, updated_at
            FROM wallets
            WHERE id = ?1
            "#,
        )?;

        let mut rows = stmt.query([id.to_string()])?;
        if let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let scope_type: String = row.get(1)?;
            let family_id: Option<String> = row.get(2)?;
            let user_id: Option<String> = row.get(3)?;
            let channel_str: String = row.get(4)?;
            let name: String = row.get(5)?;
            let account_number: Option<String> = row.get(6)?;
            let currency: String = row.get(7)?;
            let balance_str: String = row.get(8)?;
            let custodian_id_str: Option<String> = row.get(9)?;
            let is_active_int: i32 = row.get(10)?;
            let created_at_str: String = row.get(11)?;
            let updated_at_str: String = row.get(12)?;

            let channel = match channel_str.as_str() {
                "bank" => WalletChannel::Bank,
                "momo" => WalletChannel::MobileMoney,
                _ => WalletChannel::CashInHand,
            };

            let scope = if scope_type == "family" {
                LedgerScope::Family {
                    family_id: family_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal {
                    user_id: user_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            };

            let balance = Decimal::from_str(&balance_str).unwrap_or(Decimal::ZERO);
            let custodian_id = custodian_id_str.and_then(|s| Uuid::from_str(&s).ok());

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(Some(Wallet {
                id: Uuid::from_str(&id).unwrap_or_default(),
                scope,
                channel,
                name,
                account_number,
                currency,
                balance,
                custodian_id,
                is_active: is_active_int == 1,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_wallets(&self, scope: &LedgerScope) -> Result<Vec<Wallet>> {
        let (scope_type, fid_str, uid_str) = match scope {
            LedgerScope::Personal { user_id } => ("personal", None, Some(user_id.to_string())),
            LedgerScope::Family { family_id } => ("family", Some(family_id.to_string()), None),
        };

        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, scope_type, family_id, user_id, channel, name,
                   account_number, currency, balance, custodian_id, is_active, created_at, updated_at
            FROM wallets
            WHERE scope_type = ?1
              AND (?2 IS NULL OR family_id = ?2)
              AND (?3 IS NULL OR user_id = ?3)
            ORDER BY name ASC
            "#,
        )?;

        let rows = stmt.query_map(params![scope_type, fid_str, uid_str], |row| {
            let id: String = row.get(0)?;
            let scope_type: String = row.get(1)?;
            let family_id: Option<String> = row.get(2)?;
            let user_id: Option<String> = row.get(3)?;
            let channel_str: String = row.get(4)?;
            let name: String = row.get(5)?;
            let account_number: Option<String> = row.get(6)?;
            let currency: String = row.get(7)?;
            let balance_str: String = row.get(8)?;
            let custodian_id_str: Option<String> = row.get(9)?;
            let is_active_int: i32 = row.get(10)?;
            let created_at_str: String = row.get(11)?;
            let updated_at_str: String = row.get(12)?;

            let channel = match channel_str.as_str() {
                "bank" => WalletChannel::Bank,
                "momo" => WalletChannel::MobileMoney,
                _ => WalletChannel::CashInHand,
            };

            let scope = if scope_type == "family" {
                LedgerScope::Family {
                    family_id: family_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal {
                    user_id: user_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            };

            let balance = Decimal::from_str(&balance_str).unwrap_or(Decimal::ZERO);
            let custodian_id = custodian_id_str.and_then(|s| Uuid::from_str(&s).ok());

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(Wallet {
                id: Uuid::from_str(&id).unwrap_or_default(),
                scope,
                channel,
                name,
                account_number,
                currency,
                balance,
                custodian_id,
                is_active: is_active_int == 1,
                created_at,
                updated_at,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // ================= TRANSACTIONS =================

    pub fn save_transaction(&self, tx: &Transaction) -> Result<()> {
        let (scope_type, fid_str, uid_str) = match tx.scope {
            LedgerScope::Personal { user_id } => ("personal", None, user_id.to_string()),
            LedgerScope::Family { family_id } => (
                "family",
                Some(family_id.to_string()),
                tx.authorized_by.to_string(),
            ),
        };

        let tx_type_str = match tx.tx_type {
            TransactionType::Income => "income",
            TransactionType::Expense => "expense",
            TransactionType::Handover => "handover",
        };

        self.db.conn().execute(
            r#"
            INSERT INTO transactions (
                id, scope_type, family_id, user_id, source_wallet_id, destination_wallet_id,
                tx_type, amount, currency, category, description, authorized_by, paid_to,
                receipt_url, tx_date, created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, NULL, ?14, ?15)
            ON CONFLICT(id) DO NOTHING
            "#,
            params![
                tx.id.to_string(),
                scope_type,
                fid_str,
                uid_str,
                tx.source_wallet_id.to_string(),
                tx.destination_wallet_id.map(|u| u.to_string()),
                tx_type_str,
                tx.amount.to_string(),
                tx.currency,
                tx.category,
                tx.description,
                tx.authorized_by.to_string(),
                tx.paid_to,
                tx.tx_date.to_rfc3339(),
                tx.created_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn list_transactions(&self, scope: &LedgerScope, limit: usize) -> Result<Vec<Transaction>> {
        let (scope_type, fid_str, uid_str) = match scope {
            LedgerScope::Personal { user_id } => ("personal", None, Some(user_id.to_string())),
            LedgerScope::Family { family_id } => ("family", Some(family_id.to_string()), None),
        };

        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, scope_type, family_id, user_id, source_wallet_id, destination_wallet_id,
                   tx_type, amount, currency, category, description, authorized_by, paid_to,
                   receipt_url, tx_date, created_at
            FROM transactions
            WHERE scope_type = ?1
              AND (?2 IS NULL OR family_id = ?2)
              AND (?3 IS NULL OR user_id = ?3)
            ORDER BY tx_date DESC
            LIMIT ?4
            "#,
        )?;

        let rows = stmt.query_map(params![scope_type, fid_str, uid_str, limit as i64], |row| {
            let id: String = row.get(0)?;
            let scope_type: String = row.get(1)?;
            let family_id: Option<String> = row.get(2)?;
            let user_id: Option<String> = row.get(3)?;
            let source_wallet_id: String = row.get(4)?;
            let destination_wallet_id: Option<String> = row.get(5)?;
            let tx_type_str: String = row.get(6)?;
            let amount_str: String = row.get(7)?;
            let currency: String = row.get(8)?;
            let category: String = row.get(9)?;
            let description: Option<String> = row.get(10)?;
            let authorized_by: String = row.get(11)?;
            let paid_to: Option<String> = row.get(12)?;
            let _receipt_url: Option<String> = row.get(13)?;
            let tx_date_str: String = row.get(14)?;
            let created_at_str: String = row.get(15)?;

            let tx_type = match tx_type_str.as_str() {
                "income" => TransactionType::Income,
                "expense" => TransactionType::Expense,
                _ => TransactionType::Handover,
            };

            let scope = if scope_type == "family" {
                LedgerScope::Family {
                    family_id: family_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal {
                    user_id: user_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            };

            let amount = Decimal::from_str(&amount_str).unwrap_or(Decimal::ZERO);

            let tx_date = DateTime::parse_from_rfc3339(&tx_date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(Transaction {
                id: Uuid::from_str(&id).unwrap_or_default(),
                scope,
                source_wallet_id: Uuid::from_str(&source_wallet_id).unwrap_or_default(),
                destination_wallet_id: destination_wallet_id.and_then(|s| Uuid::from_str(&s).ok()),
                tx_type,
                amount,
                currency,
                category,
                description,
                authorized_by: Uuid::from_str(&authorized_by).unwrap_or_default(),
                paid_to,
                tx_date,
                created_at,
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // ================= PROJECTS =================

    pub fn save_project(&self, project: &Project) -> Result<()> {
        let status_str = match project.status {
            ProjectStatus::Planning => "planning",
            ProjectStatus::InProgress => "in_progress",
            ProjectStatus::Completed => "completed",
            ProjectStatus::Paused => "paused",
        };

        self.db.conn().execute(
            r#"
            INSERT INTO projects (
                id, family_id, name, description, target_amount, allocated_amount,
                status, start_date, target_end_date, created_by, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                target_amount = excluded.target_amount,
                allocated_amount = excluded.allocated_amount,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
            params![
                project.id.to_string(),
                project.family_id.to_string(),
                project.name,
                project.description,
                project.target_amount.to_string(),
                project.allocated_amount.to_string(),
                status_str,
                project.start_date.map(|d| d.to_string()),
                project.target_end_date.map(|d| d.to_string()),
                project.created_by.to_string(),
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn list_projects(&self, family_id: Uuid) -> Result<Vec<Project>> {
        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, family_id, name, description, target_amount, allocated_amount,
                   status, start_date, target_end_date, created_by, created_at, updated_at
            FROM projects
            WHERE family_id = ?1
            ORDER BY created_at DESC
            "#,
        )?;

        let rows = stmt.query_map([family_id.to_string()], |row| {
            let id: String = row.get(0)?;
            let fid: String = row.get(1)?;
            let name: String = row.get(2)?;
            let description: Option<String> = row.get(3)?;
            let target_str: String = row.get(4)?;
            let allocated_str: String = row.get(5)?;
            let status_str: String = row.get(6)?;
            let start_date_str: Option<String> = row.get(7)?;
            let target_end_str: Option<String> = row.get(8)?;
            let created_by: String = row.get(9)?;
            let created_at_str: String = row.get(10)?;
            let updated_at_str: String = row.get(11)?;

            let status = match status_str.as_str() {
                "in_progress" => ProjectStatus::InProgress,
                "completed" => ProjectStatus::Completed,
                "paused" => ProjectStatus::Paused,
                _ => ProjectStatus::Planning,
            };

            Ok(Project {
                id: Uuid::from_str(&id).unwrap_or_default(),
                family_id: Uuid::from_str(&fid).unwrap_or_default(),
                name,
                description,
                target_amount: Decimal::from_str(&target_str).unwrap_or(Decimal::ZERO),
                allocated_amount: Decimal::from_str(&allocated_str).unwrap_or(Decimal::ZERO),
                status,
                start_date: start_date_str
                    .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                target_end_date: target_end_str
                    .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                created_by: Uuid::from_str(&created_by).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // ================= LOANS =================

    pub fn save_loan(&self, loan: &Loan) -> Result<()> {
        let (scope_type, fid_str, uid_str) = match loan.scope {
            LedgerScope::Personal { user_id } => ("personal", None, user_id.to_string()),
            LedgerScope::Family { family_id } => (
                "family",
                Some(family_id.to_string()),
                Uuid::nil().to_string(),
            ),
        };

        let direction_str = match loan.direction {
            LoanDirection::Borrowed => "borrowed",
            LoanDirection::Lent => "lent",
        };

        let status_str = match loan.status {
            LoanStatus::Active => "active",
            LoanStatus::Repaid => "repaid",
            LoanStatus::Defaulted => "defaulted",
        };

        self.db.conn().execute(
            r#"
            INSERT INTO loans (
                id, scope_type, family_id, user_id, counterparty_name, direction,
                principal_amount, balance_remaining, interest_rate, status, due_date,
                notes, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            ON CONFLICT(id) DO UPDATE SET
                balance_remaining = excluded.balance_remaining,
                status = excluded.status,
                updated_at = excluded.updated_at
            "#,
            params![
                loan.id.to_string(),
                scope_type,
                fid_str,
                uid_str,
                loan.counterparty_name,
                direction_str,
                loan.principal_amount.to_string(),
                loan.balance_remaining.to_string(),
                loan.interest_rate.to_string(),
                status_str,
                loan.due_date.map(|d| d.to_string()),
                loan.notes,
                loan.created_at.to_rfc3339(),
                loan.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn list_loans(&self, scope: &LedgerScope) -> Result<Vec<Loan>> {
        let (scope_type, fid_str, uid_str) = match scope {
            LedgerScope::Personal { user_id } => ("personal", None, Some(user_id.to_string())),
            LedgerScope::Family { family_id } => ("family", Some(family_id.to_string()), None),
        };

        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, scope_type, family_id, user_id, counterparty_name, direction,
                   principal_amount, balance_remaining, interest_rate, status, due_date,
                   notes, created_at, updated_at
            FROM loans
            WHERE scope_type = ?1
              AND (?2 IS NULL OR family_id = ?2)
              AND (?3 IS NULL OR user_id = ?3)
            ORDER BY created_at DESC
            "#,
        )?;

        let rows = stmt.query_map(params![scope_type, fid_str, uid_str], |row| {
            let id: String = row.get(0)?;
            let scope_type: String = row.get(1)?;
            let family_id: Option<String> = row.get(2)?;
            let user_id: Option<String> = row.get(3)?;
            let counterparty: String = row.get(4)?;
            let direction_str: String = row.get(5)?;
            let principal_str: String = row.get(6)?;
            let remaining_str: String = row.get(7)?;
            let interest_str: String = row.get(8)?;
            let status_str: String = row.get(9)?;
            let due_date_str: Option<String> = row.get(10)?;
            let notes: Option<String> = row.get(11)?;
            let created_at_str: String = row.get(12)?;
            let updated_at_str: String = row.get(13)?;

            let scope = if scope_type == "family" {
                LedgerScope::Family {
                    family_id: family_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal {
                    user_id: user_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            };

            let direction = if direction_str == "lent" {
                LoanDirection::Lent
            } else {
                LoanDirection::Borrowed
            };

            let status = match status_str.as_str() {
                "repaid" => LoanStatus::Repaid,
                "defaulted" => LoanStatus::Defaulted,
                _ => LoanStatus::Active,
            };

            Ok(Loan {
                id: Uuid::from_str(&id).unwrap_or_default(),
                scope,
                counterparty_name: counterparty,
                direction,
                principal_amount: Decimal::from_str(&principal_str).unwrap_or(Decimal::ZERO),
                balance_remaining: Decimal::from_str(&remaining_str).unwrap_or(Decimal::ZERO),
                interest_rate: Decimal::from_str(&interest_str).unwrap_or(Decimal::ZERO),
                status,
                due_date: due_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                notes,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // ================= FUTURE PLANS =================

    pub fn save_future_plan(&self, plan: &FuturePlan) -> Result<()> {
        let (scope_type, fid_str, uid_str) = match plan.scope {
            LedgerScope::Personal { user_id } => ("personal", None, user_id.to_string()),
            LedgerScope::Family { family_id } => (
                "family",
                Some(family_id.to_string()),
                Uuid::nil().to_string(),
            ),
        };

        self.db.conn().execute(
            r#"
            INSERT INTO future_plans (
                id, scope_type, family_id, user_id, title, description,
                estimated_cost, target_date, is_achieved, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                description = excluded.description,
                estimated_cost = excluded.estimated_cost,
                is_achieved = excluded.is_achieved,
                updated_at = excluded.updated_at
            "#,
            params![
                plan.id.to_string(),
                scope_type,
                fid_str,
                uid_str,
                plan.title,
                plan.description,
                plan.estimated_cost.to_string(),
                plan.target_date.map(|d| d.to_string()),
                if plan.is_achieved { 1 } else { 0 },
                plan.created_at.to_rfc3339(),
                plan.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn list_future_plans(&self, scope: &LedgerScope) -> Result<Vec<FuturePlan>> {
        let (scope_type, fid_str, uid_str) = match scope {
            LedgerScope::Personal { user_id } => ("personal", None, Some(user_id.to_string())),
            LedgerScope::Family { family_id } => ("family", Some(family_id.to_string()), None),
        };

        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, scope_type, family_id, user_id, title, description,
                   estimated_cost, target_date, is_achieved, created_at, updated_at
            FROM future_plans
            WHERE scope_type = ?1
              AND (?2 IS NULL OR family_id = ?2)
              AND (?3 IS NULL OR user_id = ?3)
            ORDER BY created_at DESC
            "#,
        )?;

        let rows = stmt.query_map(params![scope_type, fid_str, uid_str], |row| {
            let id: String = row.get(0)?;
            let scope_type: String = row.get(1)?;
            let family_id: Option<String> = row.get(2)?;
            let user_id: Option<String> = row.get(3)?;
            let title: String = row.get(4)?;
            let description: Option<String> = row.get(5)?;
            let estimated_str: String = row.get(6)?;
            let target_date_str: Option<String> = row.get(7)?;
            let is_achieved_int: i32 = row.get(8)?;
            let created_at_str: String = row.get(9)?;
            let updated_at_str: String = row.get(10)?;

            let scope = if scope_type == "family" {
                LedgerScope::Family {
                    family_id: family_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal {
                    user_id: user_id
                        .and_then(|s| Uuid::from_str(&s).ok())
                        .unwrap_or_default(),
                }
            };

            Ok(FuturePlan {
                id: Uuid::from_str(&id).unwrap_or_default(),
                scope,
                title,
                description,
                estimated_cost: Decimal::from_str(&estimated_str).unwrap_or(Decimal::ZERO),
                target_date: target_date_str
                    .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
                is_achieved: is_achieved_int == 1,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    // ================= USER THEME PREFERENCES =================

    pub fn save_theme_prefs(&self, user_id: Uuid, prefs: &UserThemePrefs) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.conn().execute(
            r#"
            INSERT INTO user_theme_prefs (
                user_id, theme_mode, primary_color, primary_hover, primary_pressed,
                primary_deep, dark_canvas, dark_surface, text_muted, light_border,
                light_canvas, light_surface, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            ON CONFLICT(user_id) DO UPDATE SET
                theme_mode = excluded.theme_mode,
                primary_color = excluded.primary_color,
                primary_hover = excluded.primary_hover,
                primary_pressed = excluded.primary_pressed,
                primary_deep = excluded.primary_deep,
                dark_canvas = excluded.dark_canvas,
                dark_surface = excluded.dark_surface,
                text_muted = excluded.text_muted,
                light_border = excluded.light_border,
                light_canvas = excluded.light_canvas,
                light_surface = excluded.light_surface,
                updated_at = excluded.updated_at
            "#,
            params![
                user_id.to_string(),
                prefs.theme_mode,
                prefs.primary,
                prefs.primary_hover,
                prefs.primary_pressed,
                prefs.primary_deep,
                prefs.dark_canvas,
                prefs.dark_surface,
                prefs.text_muted,
                prefs.light_border,
                prefs.light_canvas,
                prefs.light_surface,
                now,
            ],
        )?;

        Ok(())
    }

    pub fn get_theme_prefs(&self, user_id: Uuid) -> Result<Option<UserThemePrefs>> {
        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT theme_mode, primary_color, primary_hover, primary_pressed,
                   primary_deep, dark_canvas, dark_surface, text_muted,
                   light_border, light_canvas, light_surface
            FROM user_theme_prefs
            WHERE user_id = ?1
            "#,
        )?;

        let mut rows = stmt.query([user_id.to_string()])?;
        if let Some(row) = rows.next()? {
            Ok(Some(UserThemePrefs {
                theme_mode: row.get(0)?,
                primary: row.get(1)?,
                primary_hover: row.get(2)?,
                primary_pressed: row.get(3)?,
                primary_deep: row.get(4)?,
                dark_canvas: row.get(5)?,
                dark_surface: row.get(6)?,
                text_muted: row.get(7)?,
                light_border: row.get(8)?,
                light_canvas: row.get(9)?,
                light_surface: row.get(10)?,
            }))
        } else {
            Ok(None)
        }
    }
}
