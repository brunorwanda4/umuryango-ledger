pub mod cache_repo;
pub mod db;
pub mod sync_queue;
pub mod sync_worker;

pub use cache_repo::{CacheRepository, UserThemePrefs};
pub use db::LocalDatabase;
pub use sync_queue::{QueuedMutation, SyncMutation, SyncQueueRepository, SyncStatus};
pub use sync_worker::{SyncWorker, SyncWorkerHandle};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    use rust_decimal::Decimal;
    use umuryango_core_domain::models::{
        FuturePlan, Loan, LoanDirection, LoanStatus, Project, ProjectStatus, Transaction,
        TransactionType, Wallet, WalletChannel,
    };
    use umuryango_core_domain::scope::LedgerScope;
    use uuid::Uuid;

    #[test]
    fn test_local_db_schema_initialization() {
        let db = LocalDatabase::open_in_memory().expect("failed to open in-memory db");
        let repo = CacheRepository::new(&db);
        let user_id = Uuid::new_v4();
        let scope = LedgerScope::Personal { user_id };

        let wallets = repo.list_wallets(&scope).expect("list wallets failed");
        assert_eq!(wallets.len(), 0);
    }

    #[test]
    fn test_offline_wallet_and_transaction_cache() {
        let db = LocalDatabase::open_in_memory().expect("failed to open db");
        let repo = CacheRepository::new(&db);

        let family_id = Uuid::new_v4();
        let scope = LedgerScope::Family { family_id };
        let wallet_id = Uuid::new_v4();
        let custodian_id = Uuid::new_v4();

        let wallet = Wallet {
            id: wallet_id,
            scope: scope.clone(),
            channel: WalletChannel::MobileMoney,
            name: "Family MoMo".to_string(),
            account_number: Some("0788123456".to_string()),
            currency: "RWF".to_string(),
            balance: Decimal::new(250000, 0),
            custodian_id: Some(custodian_id),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        repo.upsert_wallet(&wallet).expect("upsert wallet failed");
        let fetched = repo
            .get_wallet(wallet_id)
            .expect("get wallet failed")
            .expect("wallet should exist");
        assert_eq!(fetched.name, "Family MoMo");
        assert_eq!(fetched.balance, Decimal::new(250000, 0));

        let tx = Transaction {
            id: Uuid::new_v4(),
            scope: scope.clone(),
            source_wallet_id: wallet_id,
            destination_wallet_id: None,
            tx_type: TransactionType::Income,
            amount: Decimal::new(50000, 0),
            currency: "RWF".to_string(),
            category: "Salary".to_string(),
            description: Some("Monthly Contribution".to_string()),
            authorized_by: custodian_id,
            paid_to: None,
            tx_date: Utc::now(),
            created_at: Utc::now(),
        };

        repo.save_transaction(&tx).expect("save transaction failed");
        let txs = repo
            .list_transactions(&scope, 10)
            .expect("list transactions failed");
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].category, "Salary");
    }

    #[test]
    fn test_projects_loans_and_future_plans_cache() {
        let db = LocalDatabase::open_in_memory().expect("failed to open db");
        let repo = CacheRepository::new(&db);

        let family_id = Uuid::new_v4();
        let scope = LedgerScope::Family { family_id };

        let project = Project {
            id: Uuid::new_v4(),
            family_id,
            name: "Solar Installation".to_string(),
            description: Some("Solar panels for roof".to_string()),
            target_amount: Decimal::new(1200000, 0),
            allocated_amount: Decimal::new(300000, 0),
            status: ProjectStatus::InProgress,
            start_date: Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
            target_end_date: Some(NaiveDate::from_ymd_opt(2026, 6, 30).unwrap()),
            created_by: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.save_project(&project).expect("save project failed");
        let projects = repo.list_projects(family_id).expect("list projects failed");
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Solar Installation");

        let loan = Loan {
            id: Uuid::new_v4(),
            scope: scope.clone(),
            counterparty_name: "Tontine Ibina".to_string(),
            direction: LoanDirection::Borrowed,
            principal_amount: Decimal::new(500000, 0),
            balance_remaining: Decimal::new(200000, 0),
            interest_rate: Decimal::new(5, 0),
            status: LoanStatus::Active,
            due_date: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
            notes: Some("Monthly repayment 50k".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.save_loan(&loan).expect("save loan failed");
        let loans = repo.list_loans(&scope).expect("list loans failed");
        assert_eq!(loans.len(), 1);
        assert_eq!(loans[0].balance_remaining, Decimal::new(200000, 0));

        let plan = FuturePlan {
            id: Uuid::new_v4(),
            scope: scope.clone(),
            title: "Family Trip to Rubavu".to_string(),
            description: Some("Weekend getaway".to_string()),
            estimated_cost: Decimal::new(350000, 0),
            target_date: Some(NaiveDate::from_ymd_opt(2026, 8, 15).unwrap()),
            is_achieved: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.save_future_plan(&plan).expect("save plan failed");
        let plans = repo.list_future_plans(&scope).expect("list plans failed");
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].title, "Family Trip to Rubavu");
    }

    #[test]
    fn test_sync_queue_lifecycle() {
        let db = LocalDatabase::open_in_memory().expect("failed to open db");
        let queue = SyncQueueRepository::new(&db);

        let mutation = SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "personal".to_string(),
            family_id: None,
            source_wallet_id: Uuid::new_v4(),
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(12500, 0),
            category: "Groceries".to_string(),
            description: Some("Kimironko market".to_string()),
            paid_to: Some("Vendor".to_string()),
            receipt_url: None,
            tx_date: Some(Utc::now()),
        };

        let queue_id = queue.enqueue(&mutation).expect("enqueue failed");
        assert_eq!(queue.pending_count().unwrap(), 1);
        assert_eq!(queue.sync_status(true).unwrap(), SyncStatus::PendingSync(1));

        let pending = queue.list_pending(10).expect("list pending failed");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].action, "create_transaction");

        queue.mark_in_flight(queue_id).expect("mark in flight");
        queue.mark_synced(queue_id).expect("mark synced");
        assert_eq!(queue.pending_count().unwrap(), 0);
        assert_eq!(queue.sync_status(true).unwrap(), SyncStatus::Synced);
    }

    #[test]
    fn test_user_theme_preferences_persistence() {
        let db = LocalDatabase::open_in_memory().expect("failed to open db");
        let repo = CacheRepository::new(&db);
        let user_id = Uuid::new_v4();

        let prefs = UserThemePrefs {
            theme_mode: "dark".to_string(),
            crimson_deep: "#550000".to_string(),
            crimson_bright: "#e01020".to_string(),
            cream_warm: "#f5e6c8".to_string(),
            prussian_navy: "#001f33".to_string(),
            steel_blue: "#4682b4".to_string(),
        };

        repo.save_theme_prefs(user_id, &prefs)
            .expect("save prefs failed");
        let loaded = repo
            .get_theme_prefs(user_id)
            .expect("get prefs failed")
            .expect("prefs should exist");

        assert_eq!(loaded.theme_mode, "dark");
        assert_eq!(loaded.crimson_deep, "#550000");
    }
}
