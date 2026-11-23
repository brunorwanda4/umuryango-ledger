mod common;

use chrono::Utc;
use common::{FaultyNetworkProxy, NetworkMode, TestServer};
use rust_decimal::Decimal;
use std::time::Duration;
use tokio::time::timeout;
use umuryango_api_client::ApiClient;
use umuryango_core_domain::models::{TransactionType, Wallet, WalletChannel};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_core_domain::RealtimeEvent;
use umuryango_i18n::Locale;
use umuryango_storage_local::cache_repo::CacheRepository;
use umuryango_storage_local::sync_queue::{SyncMutation, SyncQueueRepository};
use umuryango_storage_local::sync_worker::SyncWorker;
use uuid::Uuid;

#[tokio::test]
async fn test_offline_queue_chronological_sync_and_reconciliation() {
    let server = TestServer::start().await;
    let fixture = server.seed_test_family().await;

    let (_temp_dir, local_db) = server.create_isolated_local_db();

    // 1. Seed local SQLite cache with Bruno's wallet
    {
        let locked = local_db.lock().unwrap();
        let cache = CacheRepository::new(&locked);
        cache
            .upsert_wallet(&Wallet {
                id: fixture.bruno_wallet.id,
                scope: LedgerScope::Family {
                    family_id: fixture.family_id,
                },
                channel: WalletChannel::Bank,
                name: fixture.bruno_wallet.name.clone(),
                account_number: None,
                currency: "RWF".to_string(),
                balance: fixture.bruno_wallet.initial_balance, // 150,000
                custodian_id: Some(fixture.bruno.id),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .expect("Failed to cache wallet");
    }

    // 2. Bruno logs two expenses while offline
    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);

        // Expense 1: 15,000 RWF for Cement
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(15000, 0),
            category: "Building Materials".to_string(),
            description: Some("Cement".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .expect("Failed to enqueue expense 1");

        // Expense 2: 20,000 RWF for Truck rental
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(20000, 0),
            category: "Transport".to_string(),
            description: Some("Truck rental".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .expect("Failed to enqueue expense 2");

        assert_eq!(q.pending_count().unwrap(), 2);
    }

    // 3. Connect client and flush mutations
    let client = ApiClient::new(
        server.base_url.clone(),
        server.ws_url.clone(),
        Locale::En,
    );
    client.set_token(Some(fixture.bruno.token.clone())).await;

    let flushed = SyncWorker::flush_pending_mutations(&local_db, &client).await;
    assert_eq!(flushed, 2);

    // 4. Verify SQLite queue & cache reconciliation
    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        assert_eq!(q.pending_count().unwrap(), 0);

        let cache = CacheRepository::new(&locked);
        let wallet = cache
            .get_wallet(fixture.bruno_wallet.id)
            .unwrap()
            .expect("Wallet not found in cache");
        // 150,000 - 15,000 - 20,000 = 115,000 RWF
        assert_eq!(wallet.balance, Decimal::new(115000, 0));
    }

    // 5. Verify PostgreSQL database state
    let pg_balance = sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
        .bind(fixture.bruno_wallet.id)
        .fetch_one(&server.pool)
        .await
        .unwrap();
    assert_eq!(pg_balance, Decimal::new(115000, 0));

    let tx_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM transactions WHERE family_id = $1")
            .bind(fixture.family_id)
            .fetch_one(&server.pool)
            .await
            .unwrap();
    assert_eq!(tx_count, 2);
}

#[tokio::test]
async fn test_mid_sync_dropout_idempotency_prevents_double_charge() {
    let server = TestServer::start().await;
    let fixture = server.seed_test_family().await;

    let target_addr: std::net::SocketAddr =
        format!("127.0.0.1:{}", server.port).parse().unwrap();
    let proxy = FaultyNetworkProxy::new(target_addr).await;

    let (_temp_dir, local_db) = server.create_isolated_local_db();
    {
        let locked = local_db.lock().unwrap();
        let cache = CacheRepository::new(&locked);
        cache
            .upsert_wallet(&Wallet {
                id: fixture.bruno_wallet.id,
                scope: LedgerScope::Family {
                    family_id: fixture.family_id,
                },
                channel: WalletChannel::Bank,
                name: fixture.bruno_wallet.name.clone(),
                account_number: None,
                currency: "RWF".to_string(),
                balance: Decimal::new(150000, 0),
                custodian_id: Some(fixture.bruno.id),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .unwrap();
    }

    // Configure proxy to drop connection immediately after forwarding the HTTP request
    proxy.set_mode(NetworkMode::DropAfterForwardOnce);

    let idem_key = Uuid::new_v4();
    let mutation_id = {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: idem_key,
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(30000, 0),
            category: "Emergency Repair".to_string(),
            description: Some("Plumbing fix".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap()
    };

    let client = ApiClient::new(proxy.url(), server.ws_url.clone(), Locale::En);
    client.set_token(Some(fixture.bruno.token.clone())).await;

    // First attempt: Server executes and commits to PG, but proxy drops client connection
    SyncWorker::flush_pending_mutations(&local_db, &client).await;

    // Mutation remains pending in local queue because client received transport error
    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        let item = q.get(mutation_id).unwrap().unwrap();
        assert_eq!(item.status, "pending");
    }

    // Ensure proxy is restored to Online for retry
    proxy.set_mode(NetworkMode::Online);

    // Second attempt: Client reconnects and retries with the SAME idempotency key
    SyncWorker::flush_pending_mutations(&local_db, &client).await;

    // Verify local queue is now marked synced and local cache updated
    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        let item = q.get(mutation_id).unwrap().unwrap();
        assert_eq!(
            item.status, "synced",
            "Item status should be synced, got: {} with err: {:?}",
            item.status, item.error_message
        );

        let cache = CacheRepository::new(&locked);
        let w = cache.get_wallet(fixture.bruno_wallet.id).unwrap().unwrap();
        assert_eq!(w.balance, Decimal::new(120000, 0));
    }

    // CRITICAL: Ensure balance was deducted EXACTLY ONCE (150,000 - 30,000 = 120,000, NOT 90,000)
    let pg_balance = sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
        .bind(fixture.bruno_wallet.id)
        .fetch_one(&server.pool)
        .await
        .unwrap();
    assert_eq!(pg_balance, Decimal::new(120000, 0));

    let tx_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM transactions WHERE idempotency_key = $1",
    )
    .bind(idem_key)
    .fetch_one(&server.pool)
    .await
    .unwrap();
    assert_eq!(tx_count, 1);
}

#[tokio::test]
async fn test_concurrent_custody_conflict_rolls_back_with_bilingual_error() {
    let server = TestServer::start().await;
    let fixture = server.seed_test_family().await;

    let (_temp_dir, local_db) = server.create_isolated_local_db();

    // Scenario A: English client requests amount exceeding custody balance
    let client_en = ApiClient::new(
        server.base_url.clone(),
        server.ws_url.clone(),
        Locale::En,
    );
    client_en.set_token(Some(fixture.bruno.token.clone())).await;

    let mut_id_en = {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(500000, 0), // available is only 150,000
            category: "Major Purchase".to_string(),
            description: Some("Exceeds balance".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap()
    };

    SyncWorker::flush_pending_mutations(&local_db, &client_en).await;

    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        let item = q.get(mut_id_en).unwrap().unwrap();
        assert_eq!(item.status, "failed");
        let err = item.error_message.unwrap();
        assert!(
            err.contains("INSUFFICIENT_CUSTODY") && err.contains("not enough for"),
            "Expected English error containing 'not enough for', got: {}",
            err
        );
    }

    // Scenario B: Kinyarwanda client requests amount exceeding custody balance
    let client_rw = ApiClient::new(
        server.base_url.clone(),
        server.ws_url.clone(),
        Locale::Rw,
    );
    client_rw.set_token(Some(fixture.bruno.token.clone())).await;

    let mut_id_rw = {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(600000, 0),
            category: "Ikintu Kinini".to_string(),
            description: Some("Birenze amafaranga ahari".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap()
    };

    SyncWorker::flush_pending_mutations(&local_db, &client_rw).await;

    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        let item = q.get(mut_id_rw).unwrap().unwrap();
        assert_eq!(item.status, "failed");
        let err = item.error_message.unwrap();
        assert!(
            err.contains("INSUFFICIENT_CUSTODY") && err.contains("ntihagije"),
            "Expected Kinyarwanda error containing 'ntihagije', got: {}",
            err
        );
    }

    // Ensure PostgreSQL rolled back cleanly with no balance deduction
    let pg_balance = sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
        .bind(fixture.bruno_wallet.id)
        .fetch_one(&server.pool)
        .await
        .unwrap();
    assert_eq!(pg_balance, Decimal::new(150000, 0));

    let tx_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM transactions WHERE family_id = $1")
            .bind(fixture.family_id)
            .fetch_one(&server.pool)
            .await
            .unwrap();
    assert_eq!(tx_count, 0);
}

#[tokio::test]
async fn test_intermittent_flaky_network_batch_recovery() {
    let server = TestServer::start().await;
    let fixture = server.seed_test_family().await;

    let target_addr: std::net::SocketAddr =
        format!("127.0.0.1:{}", server.port).parse().unwrap();
    let proxy = FaultyNetworkProxy::new(target_addr).await;

    let (_temp_dir, local_db) = server.create_isolated_local_db();

    // Cache initial wallets
    {
        let locked = local_db.lock().unwrap();
        let cache = CacheRepository::new(&locked);
        cache
            .upsert_wallet(&Wallet {
                id: fixture.bruno_wallet.id,
                scope: LedgerScope::Family {
                    family_id: fixture.family_id,
                },
                channel: WalletChannel::Bank,
                name: fixture.bruno_wallet.name.clone(),
                account_number: None,
                currency: "RWF".to_string(),
                balance: Decimal::new(150000, 0),
                custodian_id: Some(fixture.bruno.id),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .unwrap();

        cache
            .upsert_wallet(&Wallet {
                id: fixture.brother_wallet.id,
                scope: LedgerScope::Family {
                    family_id: fixture.family_id,
                },
                channel: WalletChannel::CashInHand,
                name: fixture.brother_wallet.name.clone(),
                account_number: None,
                currency: "RWF".to_string(),
                balance: Decimal::new(50000, 0),
                custodian_id: Some(fixture.brother.id),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .unwrap();
    }

    // 1. Set network to Offline
    proxy.set_mode(NetworkMode::Offline);

    // 2. Queue 4 mutations
    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);

        // a) Income 50,000 RWF
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Income,
            amount: Decimal::new(50000, 0),
            category: "Harvest Income".to_string(),
            description: Some("Irish potatoes harvest".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap();

        // b) Expense 10,000 RWF
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(10000, 0),
            category: "Farm Supplies".to_string(),
            description: Some("Fertilizer bags".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap();

        // c) Handover 15,000 RWF from Bruno Bank to Brother Cash
        q.enqueue(&SyncMutation::CreateHandover {
            idempotency_key: Uuid::new_v4(),
            family_id: fixture.family_id,
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: fixture.brother_wallet.id,
            amount: Decimal::new(15000, 0),
            notes: Some("Weekly market cash handover".to_string()),
        })
        .unwrap();

        // d) Expense 5,000 RWF from Bruno Bank
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(5000, 0),
            category: "Airtime".to_string(),
            description: Some("Modem bundle".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap();

        assert_eq!(q.pending_count().unwrap(), 4);
    }

    let client = ApiClient::new(proxy.url(), server.ws_url.clone(), Locale::En);
    client.set_token(Some(fixture.bruno.token.clone())).await;

    // 3. Attempt sync while offline -> mutations remain pending
    SyncWorker::flush_pending_mutations(&local_db, &client).await;

    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        assert_eq!(q.pending_count().unwrap(), 4);
    }

    // 4. Restore network to Online and flush
    proxy.set_mode(NetworkMode::Online);
    let flushed = SyncWorker::flush_pending_mutations(&local_db, &client).await;
    assert_eq!(flushed, 4);

    // 5. Verify all mutations synced
    {
        let locked = local_db.lock().unwrap();
        let q = SyncQueueRepository::new(&locked);
        assert_eq!(q.pending_count().unwrap(), 0);

        let cache = CacheRepository::new(&locked);
        let bruno_w = cache.get_wallet(fixture.bruno_wallet.id).unwrap().unwrap();
        let brother_w = cache.get_wallet(fixture.brother_wallet.id).unwrap().unwrap();

        // Bruno: 150k + 50k (income) - 10k (expense) - 15k (handover) - 5k (expense) = 170k
        assert_eq!(bruno_w.balance, Decimal::new(170000, 0));
        // Brother: 50k + 15k (handover) = 65k
        assert_eq!(brother_w.balance, Decimal::new(65000, 0));
    }

    // Verify PostgreSQL
    let pg_bruno = sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
        .bind(fixture.bruno_wallet.id)
        .fetch_one(&server.pool)
        .await
        .unwrap();
    assert_eq!(pg_bruno, Decimal::new(170000, 0));

    let pg_brother = sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
        .bind(fixture.brother_wallet.id)
        .fetch_one(&server.pool)
        .await
        .unwrap();
    assert_eq!(pg_brother, Decimal::new(65000, 0));
}

#[tokio::test]
async fn test_realtime_websocket_notification_fires_when_offline_income_syncs() {
    let server = TestServer::start().await;
    let fixture = server.seed_test_family().await;

    // Mother subscribes to the live FamilyHub for her family
    let mut mother_rx = server.family_hub.subscribe(fixture.family_id);

    let (_temp_dir, local_db) = server.create_isolated_local_db();
    {
        let locked = local_db.lock().unwrap();
        let cache = CacheRepository::new(&locked);
        cache
            .upsert_wallet(&Wallet {
                id: fixture.bruno_wallet.id,
                scope: LedgerScope::Family {
                    family_id: fixture.family_id,
                },
                channel: WalletChannel::Bank,
                name: fixture.bruno_wallet.name.clone(),
                account_number: None,
                currency: "RWF".to_string(),
                balance: Decimal::new(150000, 0),
                custodian_id: Some(fixture.bruno.id),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .unwrap();

        let q = SyncQueueRepository::new(&locked);
        q.enqueue(&SyncMutation::CreateTransaction {
            idempotency_key: Uuid::new_v4(),
            scope: "family".to_string(),
            family_id: Some(fixture.family_id),
            source_wallet_id: fixture.bruno_wallet.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Income,
            amount: Decimal::new(70000, 0),
            category: "Agribusiness".to_string(),
            description: Some("Crop Harvest Sale".to_string()),
            paid_to: None,
            receipt_url: None,
            tx_date: Some(Utc::now()),
        })
        .unwrap();
    }

    let client = ApiClient::new(
        server.base_url.clone(),
        server.ws_url.clone(),
        Locale::En,
    );
    client.set_token(Some(fixture.bruno.token.clone())).await;

    // Bruno flushes offline income to live server
    let flushed = SyncWorker::flush_pending_mutations(&local_db, &client).await;
    assert_eq!(flushed, 1);

    // Mother immediately receives RealtimeEvent on FamilyHub
    let received = timeout(Duration::from_millis(500), mother_rx.recv())
        .await
        .expect("Timed out waiting for realtime event")
        .expect("Channel closed unexpectedly");

    match &received {
        RealtimeEvent::IncomeAdded {
            amount,
            wallet_name,
            added_by_name,
            source_description,
            new_wallet_balance,
            ..
        } => {
            assert_eq!(*amount, Decimal::new(70000, 0));
            assert_eq!(wallet_name, "BK Family Custody");
            assert!(added_by_name.contains("Bruno"));
            assert_eq!(source_description, "Crop Harvest Sale");
            assert_eq!(*new_wallet_balance, Decimal::new(220000, 0));
        }
        other => panic!("Expected IncomeAdded event, got {:?}", other),
    }

    // Verify bilingual notification rendering
    let en_msg = received.format_notification(Locale::En);
    assert_eq!(
        en_msg,
        "Bruno (Bank Custodian) added 70000 RWF to BK Family Custody (Crop Harvest Sale)."
    );

    let rw_msg = received.format_notification(Locale::Rw);
    assert_eq!(
        rw_msg,
        "Bruno (Bank Custodian) yongeye 70000 RWF kuri konti 'BK Family Custody' (Crop Harvest Sale)."
    );
}
