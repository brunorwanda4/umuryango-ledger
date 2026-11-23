use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};
use umuryango_api_client::{ApiClient, CreateTransactionRequest};
use umuryango_core_domain::scope::LedgerScope;

use crate::cache_repo::CacheRepository;
use crate::db::LocalDatabase;
use crate::sync_queue::{QueuedMutation, SyncMutation, SyncQueueRepository};

pub struct SyncWorkerHandle {
    is_running: Arc<AtomicBool>,
}

impl SyncWorkerHandle {
    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

pub struct SyncWorker;

impl SyncWorker {
    /// Starts a background synchronization worker that flushes queued offline mutations
    /// and periodically pulls fresh remote data from the Actix-web server.
    pub fn start(
        db: Arc<std::sync::Mutex<LocalDatabase>>,
        client: ApiClient,
        scope: LedgerScope,
        poll_interval: Duration,
    ) -> SyncWorkerHandle {
        let is_running = Arc::new(AtomicBool::new(true));
        let running_clone = is_running.clone();

        tokio::spawn(async move {
            info!("SyncWorker started with poll interval {:?}", poll_interval);

            while running_clone.load(Ordering::SeqCst) {
                // 1. Flush pending offline queue items
                let pending = {
                    let locked = db.lock().unwrap();
                    let queue_repo = SyncQueueRepository::new(&locked);
                    queue_repo.list_pending(20).unwrap_or_default()
                };

                for item in pending {
                    if !running_clone.load(Ordering::SeqCst) {
                        break;
                    }
                    Self::process_mutation(&db, &client, &item).await;
                }

                // 2. Pull authoritative remote state when online with authenticated token
                if client.get_token().await.is_some() {
                    Self::pull_remote_state(&db, &client, &scope).await;
                }

                sleep(poll_interval).await;
            }

            info!("SyncWorker stopped.");
        });

        SyncWorkerHandle { is_running }
    }

    pub async fn process_mutation(
        db: &Arc<std::sync::Mutex<LocalDatabase>>,
        client: &ApiClient,
        item: &QueuedMutation,
    ) {
        {
            let locked = db.lock().unwrap();
            let repo = SyncQueueRepository::new(&locked);
            let _ = repo.mark_in_flight(item.id);
        }

        match &item.mutation {
            SyncMutation::CreateTransaction {
                idempotency_key,
                scope,
                family_id,
                source_wallet_id,
                destination_wallet_id,
                tx_type,
                amount,
                category,
                description,
                paid_to,
                receipt_url,
                tx_date,
            } => {
                let req = CreateTransactionRequest {
                    scope: scope.clone(),
                    family_id: *family_id,
                    source_wallet_id: *source_wallet_id,
                    destination_wallet_id: *destination_wallet_id,
                    tx_type: *tx_type,
                    amount: *amount,
                    category: category.clone(),
                    description: description.clone(),
                    paid_to: paid_to.clone(),
                    receipt_url: receipt_url.clone(),
                    tx_date: *tx_date,
                    idempotency_key: Some(*idempotency_key),
                };

                match client
                    .create_transaction(&req, Some(*idempotency_key))
                    .await
                {
                    Ok(result) => {
                        let locked = db.lock().unwrap();
                        let q_repo = SyncQueueRepository::new(&locked);
                        let cache_repo = CacheRepository::new(&locked);

                        let _ = q_repo.mark_synced(item.id);
                        let _ = cache_repo
                            .update_wallet_balance(*source_wallet_id, result.source_wallet_balance);
                        if let (Some(did), Some(dbal)) =
                            (*destination_wallet_id, result.destination_wallet_balance)
                        {
                            let _ = cache_repo.update_wallet_balance(did, dbal);
                        }
                    }
                    Err(e) => {
                        warn!("Offline mutation {} failed: {:?}", item.id, e);
                        let locked = db.lock().unwrap();
                        let q_repo = SyncQueueRepository::new(&locked);
                        let _ = q_repo.mark_failed(item.id, &e.to_string());
                    }
                }
            }
            SyncMutation::CreateHandover {
                idempotency_key,
                family_id,
                source_wallet_id,
                destination_wallet_id,
                amount,
                notes,
            } => {
                let req = CreateTransactionRequest {
                    scope: "family".to_string(),
                    family_id: Some(*family_id),
                    source_wallet_id: *source_wallet_id,
                    destination_wallet_id: Some(*destination_wallet_id),
                    tx_type: umuryango_core_domain::models::TransactionType::Handover,
                    amount: *amount,
                    category: "Handover".to_string(),
                    description: notes.clone(),
                    paid_to: None,
                    receipt_url: None,
                    tx_date: None,
                    idempotency_key: Some(*idempotency_key),
                };

                match client
                    .create_transaction(&req, Some(*idempotency_key))
                    .await
                {
                    Ok(result) => {
                        let locked = db.lock().unwrap();
                        let q_repo = SyncQueueRepository::new(&locked);
                        let cache_repo = CacheRepository::new(&locked);

                        let _ = q_repo.mark_synced(item.id);
                        let _ = cache_repo
                            .update_wallet_balance(*source_wallet_id, result.source_wallet_balance);
                        if let Some(dbal) = result.destination_wallet_balance {
                            let _ = cache_repo.update_wallet_balance(*destination_wallet_id, dbal);
                        }
                    }
                    Err(e) => {
                        warn!("Handover mutation {} failed: {:?}", item.id, e);
                        let locked = db.lock().unwrap();
                        let q_repo = SyncQueueRepository::new(&locked);
                        let _ = q_repo.mark_failed(item.id, &e.to_string());
                    }
                }
            }
            SyncMutation::RepayLoan {
                loan_id, amount, ..
            } => match client.repay_loan(*loan_id, *amount).await {
                Ok(_) => {
                    let locked = db.lock().unwrap();
                    let q_repo = SyncQueueRepository::new(&locked);
                    let _ = q_repo.mark_synced(item.id);
                }
                Err(e) => {
                    let locked = db.lock().unwrap();
                    let q_repo = SyncQueueRepository::new(&locked);
                    let _ = q_repo.mark_failed(item.id, &e.to_string());
                }
            },
            SyncMutation::AllocatePlan {
                plan_id, amount, ..
            } => match client.allocate_plan(*plan_id, *amount).await {
                Ok(_) => {
                    let locked = db.lock().unwrap();
                    let q_repo = SyncQueueRepository::new(&locked);
                    let _ = q_repo.mark_synced(item.id);
                }
                Err(e) => {
                    let locked = db.lock().unwrap();
                    let q_repo = SyncQueueRepository::new(&locked);
                    let _ = q_repo.mark_failed(item.id, &e.to_string());
                }
            },
        }
    }

    pub async fn pull_remote_state(
        db: &Arc<std::sync::Mutex<LocalDatabase>>,
        client: &ApiClient,
        scope: &LedgerScope,
    ) {
        // 1. Wallets
        if let Ok(wallets) = client.list_wallets(scope).await {
            let locked = db.lock().unwrap();
            let repo = CacheRepository::new(&locked);
            for w in wallets {
                let _ = repo.upsert_wallet(&w);
            }
        }

        // 2. Transactions
        if let Ok(txs) = client.list_transactions(scope, None).await {
            let locked = db.lock().unwrap();
            let repo = CacheRepository::new(&locked);
            for t in txs {
                let _ = repo.save_transaction(&t);
            }
        }

        // 3. Projects (if Family scope)
        if let LedgerScope::Family { family_id } = scope {
            if let Ok(projects) = client.list_projects(*family_id).await {
                let locked = db.lock().unwrap();
                let repo = CacheRepository::new(&locked);
                for p in projects {
                    let _ = repo.save_project(&p);
                }
            }
        }

        // 4. Loans
        if let Ok(loans) = client.list_loans(scope).await {
            let locked = db.lock().unwrap();
            let repo = CacheRepository::new(&locked);
            for l in loans {
                let _ = repo.save_loan(&l);
            }
        }

        // 5. Future Plans
        if let Ok(plans) = client.list_future_plans(scope).await {
            let locked = db.lock().unwrap();
            let repo = CacheRepository::new(&locked);
            for pl in plans {
                let _ = repo.save_future_plan(&pl);
            }
        }
    }
}
