use chrono::Utc;
use rust_decimal::Decimal;
use tokio::time::{timeout, Duration};
use umuryango_api_server::{FamilyHub, RealtimeEvent};
use umuryango_i18n::Locale;
use uuid::Uuid;

#[tokio::test]
async fn test_family_hub_broadcast_and_isolation() {
    let hub = FamilyHub::new();

    let family_a = Uuid::new_v4();
    let family_b = Uuid::new_v4();

    // Clients 1 & 2 belong to Family A
    let mut client_1_rx = hub.subscribe(family_a);
    let mut client_2_rx = hub.subscribe(family_a);

    // Client 3 belongs to Family B
    let mut client_3_rx = hub.subscribe(family_b);

    let wallet_id = Uuid::new_v4();
    let income_event = RealtimeEvent::IncomeAdded {
        family_id: family_a,
        wallet_id,
        wallet_name: "MoMo Main".to_string(),
        amount: Decimal::new(75000, 0),
        currency: "RWF".to_string(),
        added_by_name: "Keza".to_string(),
        source_description: "Agribusiness Dividend".to_string(),
        new_wallet_balance: Decimal::new(250000, 0),
        occurred_at: Utc::now(),
    };

    // Broadcast to Family A
    hub.broadcast(family_a, income_event.clone());

    // Both clients in Family A receive the event
    let received_1 = timeout(Duration::from_millis(100), client_1_rx.recv())
        .await
        .expect("Client 1 timed out")
        .expect("Client 1 channel closed");
    assert_eq!(received_1, income_event);

    let received_2 = timeout(Duration::from_millis(100), client_2_rx.recv())
        .await
        .expect("Client 2 timed out")
        .expect("Client 2 channel closed");
    assert_eq!(received_2, income_event);

    // Client 3 in Family B must NOT receive Family A's event
    let received_3 = timeout(Duration::from_millis(50), client_3_rx.recv()).await;
    assert!(
        received_3.is_err(),
        "Client 3 must not receive events from Family A"
    );

    // Bilingual formatting tests
    let en_text = received_1.format_notification(Locale::En);
    assert_eq!(
        en_text,
        "Keza added 75000 RWF to MoMo Main (Agribusiness Dividend)."
    );

    let rw_text = received_1.format_notification(Locale::Rw);
    assert_eq!(
        rw_text,
        "Keza yongeye 75000 RWF kuri konti 'MoMo Main' (Agribusiness Dividend)."
    );
}

#[tokio::test]
async fn test_bilingual_notifications_all_event_variants() {
    let family_id = Uuid::new_v4();

    // 1. ExpenseLogged
    let expense_event = RealtimeEvent::ExpenseLogged {
        family_id,
        wallet_id: Uuid::new_v4(),
        wallet_name: "Cash Safe".to_string(),
        amount: Decimal::new(12000, 0),
        currency: "RWF".to_string(),
        logged_by_name: "Eric".to_string(),
        category: "Groceries".to_string(),
        new_wallet_balance: Decimal::new(88000, 0),
        occurred_at: Utc::now(),
    };

    let en_expense = expense_event.format_notification(Locale::En);
    assert_eq!(
        en_expense,
        "Eric spent 12000 RWF from Cash Safe on Groceries."
    );

    let rw_expense = expense_event.format_notification(Locale::Rw);
    assert_eq!(
        rw_expense,
        "Eric yakoresheje 12000 RWF kuri konti 'Cash Safe' (Groceries)."
    );

    // 2. HandoverCompleted
    let handover_event = RealtimeEvent::HandoverCompleted {
        family_id,
        from_wallet_name: "Bank Account".to_string(),
        to_wallet_name: "MoMo Main".to_string(),
        amount: Decimal::new(100000, 0),
        currency: "RWF".to_string(),
        initiated_by_name: "Daddy".to_string(),
        occurred_at: Utc::now(),
    };

    let en_handover = handover_event.format_notification(Locale::En);
    assert_eq!(
        en_handover,
        "Daddy transferred 100000 RWF from Bank Account to MoMo Main."
    );

    let rw_handover = handover_event.format_notification(Locale::Rw);
    assert_eq!(
        rw_handover,
        "Daddy yohereje 100000 RWF kuva kuri 'Bank Account' ajya kuri 'MoMo Main'."
    );

    // 3. ProjectOrLoanUpdated
    let project_event = RealtimeEvent::ProjectOrLoanUpdated {
        family_id,
        entity_type: "Project".to_string(),
        title: "Poultry Farm Phase 1".to_string(),
    };

    let en_project = project_event.format_notification(Locale::En);
    assert_eq!(en_project, "Project 'Poultry Farm Phase 1' was updated.");

    let rw_project = project_event.format_notification(Locale::Rw);
    assert_eq!(rw_project, "Project 'Poultry Farm Phase 1' yavuguruwe.");
}

#[test]
fn test_realtime_event_wire_serialization() {
    let event = RealtimeEvent::IncomeAdded {
        family_id: Uuid::new_v4(),
        wallet_id: Uuid::new_v4(),
        wallet_name: "BK Family".to_string(),
        amount: Decimal::new(250000, 0),
        currency: "RWF".to_string(),
        added_by_name: "Alice".to_string(),
        source_description: "Consulting".to_string(),
        new_wallet_balance: Decimal::new(1250000, 0),
        occurred_at: Utc::now(),
    };

    // Serialize to JSON text (as transmitted over WebSocket)
    let json = serde_json::to_string(&event).expect("Serialization failed");
    assert!(json.contains("\"type\":\"IncomeAdded\""));
    assert!(json.contains("\"amount\":\"250000\""));

    // Deserialize back
    let deserialized: RealtimeEvent = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(event, deserialized);
}
