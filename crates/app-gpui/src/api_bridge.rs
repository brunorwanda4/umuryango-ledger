//! Async bridge between GPUI's UI executor and the native Tokio/reqwest API client.
//!
//! On native targets, HTTP calls run on a dedicated Tokio multi-threaded runtime
//! and resolve back to GPUI via `tokio::sync::oneshot`, avoiding both UI thread
//! blocking and Tokio reactor panics inside GPUI's executor.

use std::collections::HashMap;
use umuryango_core_domain::models::{
    FamilyMemberView, FamilyWithRole, FuturePlan, Loan, PendingFamilyInvite, Project, Transaction,
    Wallet,
};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::{ApiClient, ApiClientError};
#[cfg(not(target_arch = "wasm32"))]
use umuryango_storage_local::{CacheRepository, LocalDatabase, SyncQueueRepository, SyncWorker};

/// Full authoritative snapshot fetched from PostgreSQL for a single scope.
#[derive(Debug, Clone)]
pub struct ScopeSnapshot {
    pub scope: LedgerScope,
    pub wallets: Vec<Wallet>,
    pub custodian_names: HashMap<Uuid, String>,
    pub transactions: Vec<Transaction>,
    pub projects: Vec<Project>,
    pub loans: Vec<Loan>,
    pub future_plans: Vec<FuturePlan>,
    pub members: Vec<FamilyMemberView>,
    pub families: Vec<FamilyWithRole>,
    pub pending_invites: Vec<PendingFamilyInvite>,
    pub pending_sync_count: usize,
}

/// Resolves the API HTTP and WebSocket URLs from environment variables or defaults.
pub fn resolve_api_urls() -> (String, String) {
    let raw = std::env::var("UMURYANGO_API_URL")
        .or_else(|_| std::env::var("API_URL"))
        .or_else(|_| std::env::var("NEXT_PUBLIC_API_URL"))
        .unwrap_or_else(|_| "http://127.0.0.1:4433".to_string());

    let trimmed = raw.trim().trim_end_matches('/');
    let base = trimmed
        .strip_suffix("/api/v1")
        .unwrap_or(trimmed)
        .to_string();

    let ws = if let Some(rest) = base.strip_prefix("https://") {
        format!("wss://{}/api/v1/ws", rest)
    } else if let Some(rest) = base.strip_prefix("http://") {
        format!("ws://{}/api/v1/ws", rest)
    } else {
        format!("ws://{}/api/v1/ws", base)
    };

    (base, ws)
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct ApiBridge {
    pub client: ApiClient,
    runtime: Arc<tokio::runtime::Runtime>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ApiBridge {
    pub fn new(locale: Locale) -> Self {
        let (base_url, ws_url) = resolve_api_urls();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(2)
            .thread_name("umuryango-api")
            .build()
            .expect("Failed to initialize Tokio runtime for API client");

        Self {
            client: ApiClient::new(base_url, ws_url, locale),
            runtime: Arc::new(runtime),
        }
    }

    pub fn base_url(&self) -> String {
        self.client.base_url().to_string()
    }

    /// Runs a Tokio-dependent future on the dedicated Tokio runtime and returns
    /// an executor-agnostic future that can be awaited inside GPUI's `cx.spawn`.
    pub fn run<T, F>(&self, fut: F) -> impl std::future::Future<Output = T> + Send + 'static
    where
        T: Send + 'static,
        F: std::future::Future<Output = T> + Send + 'static,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.runtime.spawn(async move {
            let res = fut.await;
            let _ = tx.send(res);
        });
        async move { rx.await.expect("API runtime task terminated unexpectedly") }
    }

    /// Flushes pending offline mutations (if any) and pulls fresh authoritative state
    /// from PostgreSQL for the given scope, updating the local SQLite cache.
    pub async fn fetch_scope_snapshot(
        client: ApiClient,
        scope: LedgerScope,
        db: Option<Arc<Mutex<LocalDatabase>>>,
    ) -> Result<ScopeSnapshot, ApiClientError> {
        if let Some(ref db_arc) = db {
            let _ = SyncWorker::flush_pending_mutations(db_arc, &client).await;
        }

        let detailed_wallets = client.list_wallets_detailed(&scope).await?;
        let mut wallets = Vec::with_capacity(detailed_wallets.len());
        let mut custodian_names = HashMap::new();
        for view in detailed_wallets {
            let wid = view.id;
            let (wallet, cname) = view.into_wallet();
            if let Some(name) = cname {
                custodian_names.insert(wid, name);
            }
            wallets.push(wallet);
        }

        let transactions = client.list_transactions(&scope).await?;
        let loans = client.list_loans(&scope).await?;
        let future_plans = client.list_plans(&scope).await?;
        let families = client.list_families().await.unwrap_or_default();
        let pending_invites = client.list_pending_invites().await.unwrap_or_default();

        let (projects, members) = match scope {
            LedgerScope::Family { family_id } => {
                let p = client.list_projects(Some(family_id)).await?;
                let m = client.list_family_members(family_id).await?;
                (p, m)
            }
            LedgerScope::Personal { .. } => (Vec::new(), Vec::new()),
        };

        let mut pending_sync_count = 0;
        if let Some(ref db_arc) = db
            && let Ok(locked) = db_arc.lock()
        {
            let repo = CacheRepository::new(&locked);
            for w in &wallets {
                let _ = repo.upsert_wallet(w);
            }
            for tx in &transactions {
                let _ = repo.save_transaction(tx);
            }
            for p in &projects {
                let _ = repo.save_project(p);
            }
            for l in &loans {
                let _ = repo.save_loan(l);
            }
            for pl in &future_plans {
                let _ = repo.save_future_plan(pl);
            }
            let q = SyncQueueRepository::new(&locked);
            pending_sync_count = q.pending_count().unwrap_or(0);
        }

        Ok(ScopeSnapshot {
            scope,
            wallets,
            custodian_names,
            transactions,
            projects,
            loans,
            future_plans,
            members,
            families,
            pending_invites,
            pending_sync_count,
        })
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct ApiBridge {
    base_url: String,
}

#[cfg(target_arch = "wasm32")]
impl ApiBridge {
    pub fn new(_locale: Locale) -> Self {
        let (base_url, _) = resolve_api_urls();
        Self { base_url }
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }
}
