use super::*;
use umuryango_core_domain::models::{CategoryKind, MoneyCategory};

#[test]
fn test_default_state_has_no_fake_seed_data() {
    let state = AppState::default();
    assert!(!state.is_authenticated);
    assert!(state.user_name.is_empty());
    assert!(state.active_family_name.is_none());
    assert!(state.wallets.is_empty());
    assert!(state.categories.is_empty());
    assert!(state.transactions.is_empty());
    assert!(state.family_members.is_empty());
    assert!(state.sidebar_open);
    assert!(!state.families_menu_open);
    assert!(!state.show_command_palette);
    assert!(state.active_category_dialog.is_none());
    assert_eq!(state.category_form_kind, CategoryKind::Expense);
    assert!(state.dashboard_period.is_none());
    assert_eq!(state.current_screen, Screen::Auth);
    assert_eq!(state.onboarding_step, OnboardingStep::Choose);
    assert_eq!(state.toast_id, 0);
}

#[test]
fn test_save_money_category_local_persists_and_sorts() {
    let user_id = Uuid::new_v4();
    let scope = LedgerScope::Personal { user_id };
    let mut state = AppState {
        user_id,
        active_scope: scope,
        is_authenticated: true,
        ..Default::default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    let db = LocalDatabase::open_in_memory().ok();
    #[cfg(target_arch = "wasm32")]
    let db: Option<LocalDatabase> = None;

    let cat_expense = MoneyCategory {
        id: Uuid::new_v4(),
        scope,
        kind: CategoryKind::Expense,
        name: "School Fees".to_string(),
        created_at: Utc::now(),
    };
    let cat_income = MoneyCategory {
        id: Uuid::new_v4(),
        scope,
        kind: CategoryKind::Income,
        name: "Coffee Harvest".to_string(),
        created_at: Utc::now(),
    };

    state.save_money_category_local(cat_expense, db.as_ref(), false);
    state.save_money_category_local(cat_income, db.as_ref(), true);

    assert_eq!(state.categories.len(), 2);
    assert_eq!(state.categories[0].name, "Coffee Harvest");
    assert_eq!(state.categories[0].kind, CategoryKind::Income);
    assert_eq!(state.categories[1].name, "School Fees");
    assert_eq!(state.categories[1].kind, CategoryKind::Expense);

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(ref database) = db {
        let mut reloaded = AppState {
            user_id,
            active_scope: scope,
            is_authenticated: true,
            ..Default::default()
        };
        reloaded.load_from_db(database);
        assert_eq!(reloaded.categories.len(), 2);
        assert_eq!(reloaded.categories[0].name, "Coffee Harvest");
        assert_eq!(reloaded.categories[1].name, "School Fees");
    }
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
