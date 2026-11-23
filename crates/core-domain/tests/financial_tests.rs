use chrono::Utc;
use rust_decimal::Decimal;
use umuryango_core_domain::DomainError;
use umuryango_core_domain::ledger::LedgerValidator;
use umuryango_core_domain::models::{
    FamilyRole, FuturePlan, Loan, LoanDirection, LoanStatus, Project, ProjectStatus, Transaction,
    TransactionType, Wallet, WalletChannel,
};
use umuryango_core_domain::reporting::{LocalizedExporter, generate_mothers_report};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;
use uuid::Uuid;

#[test]
fn test_error_formatting_bilingual() {
    let err = DomainError::InsufficientCustodyBalance {
        wallet: "Bruno MoMo".to_string(),
        available: Decimal::new(15000, 0),
        requested: Decimal::new(50000, 0),
    };

    // English formatting
    let localized_en = err.localize(Locale::En);
    assert_eq!(localized_en.code, "error-insufficient-custody");
    assert_eq!(
        localized_en.message,
        "Wallet 'Bruno MoMo' only holds 15000 RWF, which is not enough for 50000 RWF."
    );

    // Kinyarwanda formatting
    let localized_rw = err.localize(Locale::Rw);
    assert_eq!(localized_rw.code, "error-insufficient-custody");
    assert_eq!(
        localized_rw.message,
        "Konti 'Bruno MoMo' ifite 15000 RWF gusa; ntihagije ngo ukoreshe 50000 RWF."
    );
}

#[test]
fn test_custodian_handover_preserves_total_family_cash() {
    let family_id = Uuid::new_v4();
    let bruno_id = Uuid::new_v4();
    let brother_id = Uuid::new_v4();

    let bruno_bank = Wallet {
        id: Uuid::new_v4(),
        scope: LedgerScope::Family { family_id },
        channel: WalletChannel::Bank,
        name: "Bruno Bank Account".to_string(),
        account_number: Some("0001-BK".to_string()),
        currency: "RWF".to_string(),
        balance: Decimal::new(200000, 0),
        custodian_id: Some(bruno_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let brother_cash = Wallet {
        id: Uuid::new_v4(),
        scope: LedgerScope::Family { family_id },
        channel: WalletChannel::CashInHand,
        name: "Brother Cash Pocket".to_string(),
        account_number: None,
        currency: "RWF".to_string(),
        balance: Decimal::new(50000, 0),
        custodian_id: Some(brother_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let initial_total = bruno_bank.balance + brother_cash.balance;
    assert_eq!(initial_total, Decimal::new(250000, 0));

    // Handover 75,000 RWF from Bruno's Bank to Brother's Cash
    let handover_amount = Decimal::new(75000, 0);
    let handover_tx = Transaction {
        id: Uuid::new_v4(),
        scope: LedgerScope::Family { family_id },
        source_wallet_id: bruno_bank.id,
        destination_wallet_id: Some(brother_cash.id),
        tx_type: TransactionType::Handover,
        amount: handover_amount,
        currency: "RWF".to_string(),
        category: "Custody Handover".to_string(),
        description: Some("Transfer to brother for market purchases".to_string()),
        authorized_by: bruno_id,
        paid_to: Some("Brother".to_string()),
        tx_date: Utc::now(),
        created_at: Utc::now(),
    };

    let mut wallets = vec![bruno_bank.clone(), brother_cash.clone()];

    // Execute handover with Custodian role
    let result = LedgerValidator::validate_and_apply_transaction(
        &handover_tx,
        FamilyRole::Custodian,
        &mut wallets,
    );

    assert!(
        result.is_ok(),
        "Handover should succeed: {:?}",
        result.err()
    );

    let updated_bruno = &wallets[0];
    let updated_brother = &wallets[1];

    assert_eq!(updated_bruno.balance, Decimal::new(125000, 0));
    assert_eq!(updated_brother.balance, Decimal::new(125000, 0));

    let updated_total = updated_bruno.balance + updated_brother.balance;
    assert_eq!(
        updated_total, initial_total,
        "Total family cash MUST stay identical during internal custody handover!"
    );
}

#[test]
fn test_rejection_of_mixing_personal_and_family_wallets() {
    let user_id = Uuid::new_v4();
    let family_id = Uuid::new_v4();

    let personal_wallet = Wallet {
        id: Uuid::new_v4(),
        scope: LedgerScope::Personal { user_id },
        channel: WalletChannel::MobileMoney,
        name: "My Private MoMo".to_string(),
        account_number: Some("0788000001".to_string()),
        currency: "RWF".to_string(),
        balance: Decimal::new(100000, 0),
        custodian_id: Some(user_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let family_wallet = Wallet {
        id: Uuid::new_v4(),
        scope: LedgerScope::Family { family_id },
        channel: WalletChannel::Bank,
        name: "Family Vault".to_string(),
        account_number: Some("BK-001".to_string()),
        currency: "RWF".to_string(),
        balance: Decimal::new(500000, 0),
        custodian_id: Some(user_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let mut wallets = vec![personal_wallet.clone(), family_wallet.clone()];

    // Attempt 1: Personal transaction attempting to spend from Family wallet
    let cross_scope_tx = Transaction {
        id: Uuid::new_v4(),
        scope: LedgerScope::Personal { user_id },
        source_wallet_id: family_wallet.id, // Family wallet!
        destination_wallet_id: None,
        tx_type: TransactionType::Expense,
        amount: Decimal::new(10000, 0),
        currency: "RWF".to_string(),
        category: "Personal Food".to_string(),
        description: None,
        authorized_by: user_id,
        paid_to: None,
        tx_date: Utc::now(),
        created_at: Utc::now(),
    };

    let res1 = LedgerValidator::validate_and_apply_transaction(
        &cross_scope_tx,
        FamilyRole::Head,
        &mut wallets,
    );
    assert_eq!(res1, Err(DomainError::ScopeMismatch));

    // Attempt 2: Handover between Personal wallet and Family wallet
    let cross_handover_tx = Transaction {
        id: Uuid::new_v4(),
        scope: LedgerScope::Family { family_id },
        source_wallet_id: personal_wallet.id,
        destination_wallet_id: Some(family_wallet.id),
        tx_type: TransactionType::Handover,
        amount: Decimal::new(20000, 0),
        currency: "RWF".to_string(),
        category: "Handover".to_string(),
        description: None,
        authorized_by: user_id,
        paid_to: None,
        tx_date: Utc::now(),
        created_at: Utc::now(),
    };

    let res2 = LedgerValidator::validate_and_apply_transaction(
        &cross_handover_tx,
        FamilyRole::Head,
        &mut wallets,
    );
    assert_eq!(res2, Err(DomainError::InvalidHandover));
}

#[test]
fn test_mothers_report_calculation_accuracy() {
    let family_id = Uuid::new_v4();
    let scope = LedgerScope::Family { family_id };
    let bruno_id = Uuid::new_v4();
    let sister_id = Uuid::new_v4();

    let w1 = Wallet {
        id: Uuid::new_v4(),
        scope,
        channel: WalletChannel::Bank,
        name: "Bruno Bank".to_string(),
        account_number: Some("001".to_string()),
        currency: "RWF".to_string(),
        balance: Decimal::new(100000, 0),
        custodian_id: Some(bruno_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let w2 = Wallet {
        id: Uuid::new_v4(),
        scope,
        channel: WalletChannel::MobileMoney,
        name: "Sister MoMo".to_string(),
        account_number: Some("0788".to_string()),
        currency: "RWF".to_string(),
        balance: Decimal::new(50000, 0),
        custodian_id: Some(sister_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let w3 = Wallet {
        id: Uuid::new_v4(),
        scope,
        channel: WalletChannel::CashInHand,
        name: "Cash In Safe".to_string(),
        account_number: None,
        currency: "RWF".to_string(),
        balance: Decimal::new(30000, 0),
        custodian_id: Some(bruno_id),
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let wallets = vec![w1.clone(), w2.clone(), w3.clone()];
    // Total closing balance = 100k + 50k + 30k = 180,000 RWF

    let now = Utc::now();
    let period_start = now - chrono::Duration::days(30);
    let period_end = now + chrono::Duration::days(1);

    let transactions = vec![
        // Income +200,000
        Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: w1.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Income,
            amount: Decimal::new(200000, 0),
            currency: "RWF".to_string(),
            category: "Harvest".to_string(),
            description: Some("Coffee crop sale".to_string()),
            authorized_by: bruno_id,
            paid_to: None,
            tx_date: now - chrono::Duration::days(15),
            created_at: now - chrono::Duration::days(15),
        },
        // Expense -40,000 (Medical)
        Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: w2.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(40000, 0),
            currency: "RWF".to_string(),
            category: "Medical".to_string(),
            description: Some("Clinic medicines".to_string()),
            authorized_by: sister_id,
            paid_to: Some("Pharmacy".to_string()),
            tx_date: now - chrono::Duration::days(10),
            created_at: now - chrono::Duration::days(10),
        },
        // Expense -60,000 (Food)
        Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: w3.id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::new(60000, 0),
            currency: "RWF".to_string(),
            category: "Food".to_string(),
            description: Some("Monthly groceries".to_string()),
            authorized_by: bruno_id,
            paid_to: Some("Supermarket".to_string()),
            tx_date: now - chrono::Duration::days(5),
            created_at: now - chrono::Duration::days(5),
        },
        // Handover 30,000 (Bruno Bank -> Cash in Safe)
        // Must NOT increase or decrease income/expenses!
        Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: w1.id,
            destination_wallet_id: Some(w3.id),
            tx_type: TransactionType::Handover,
            amount: Decimal::new(30000, 0),
            currency: "RWF".to_string(),
            category: "Handover".to_string(),
            description: Some("Cash withdrawal to safe".to_string()),
            authorized_by: bruno_id,
            paid_to: None,
            tx_date: now - chrono::Duration::days(2),
            created_at: now - chrono::Duration::days(2),
        },
    ];

    let projects = vec![Project {
        id: Uuid::new_v4(),
        family_id,
        name: "Roof Renovation".to_string(),
        description: Some("Fixing homestead roof".to_string()),
        target_amount: Decimal::new(1000000, 0),
        allocated_amount: Decimal::new(250000, 0),
        status: ProjectStatus::InProgress,
        start_date: None,
        target_end_date: None,
        created_by: bruno_id,
        created_at: now,
        updated_at: now,
    }];

    let loans = vec![Loan {
        id: Uuid::new_v4(),
        scope,
        counterparty_name: "SACCO".to_string(),
        principal_amount: Decimal::new(500000, 0),
        balance_remaining: Decimal::new(150000, 0),
        interest_rate: Decimal::new(5, 0),
        due_date: None,
        direction: LoanDirection::Borrowed,
        status: LoanStatus::Active,
        notes: None,
        created_at: now,
        updated_at: now,
    }];

    let plans = vec![FuturePlan {
        id: Uuid::new_v4(),
        scope,
        title: "Cow Purchase".to_string(),
        description: Some("Buying heifer for dairy".to_string()),
        estimated_cost: Decimal::new(400000, 0),
        target_date: None,
        is_achieved: false,
        created_at: now,
        updated_at: now,
    }];

    let report = generate_mothers_report(
        scope,
        &wallets,
        &transactions,
        &projects,
        &loans,
        &plans,
        period_start,
        period_end,
    );

    // Verify mathematical accuracy
    assert_eq!(report.closing_balance, Decimal::new(180000, 0));
    assert_eq!(report.total_income, Decimal::new(200000, 0));
    assert_eq!(report.total_expenses, Decimal::new(100000, 0));
    // Opening = Closing (180,000) - Income (200,000) + Expenses (100,000) = 80,000
    assert_eq!(report.opening_balance, Decimal::new(80000, 0));

    // Verify expenses by category
    assert_eq!(
        report.expenses_by_category.get("Medical"),
        Some(&Decimal::new(40000, 0))
    );
    assert_eq!(
        report.expenses_by_category.get("Food"),
        Some(&Decimal::new(60000, 0))
    );
    assert_eq!(report.expenses_by_category.get("Handover"), None);

    // Verify custodian balances count
    assert_eq!(report.custodian_balances.len(), 3);
    assert_eq!(report.active_projects.len(), 1);
    assert_eq!(report.open_loans.len(), 1);
    assert_eq!(report.future_plans.len(), 1);

    // Verify WhatsApp bilingual export
    let whatsapp_rw = LocalizedExporter::format_mothers_report(&report, Locale::Rw);
    assert!(whatsapp_rw.contains("Incamake y'Imari y'Umuryango (Raporo y'Ababyeyi)"));
    assert!(whatsapp_rw.contains("*Asanzwe mu Ntoki / Amakonti*: 80000.00 RWF"));
    assert!(whatsapp_rw.contains("*Amafaranga Yose Yinjiye*: +200000.00 RWF"));
    assert!(whatsapp_rw.contains("*Amafaranga Yose Yasohotse*: -100000.00 RWF"));
    assert!(whatsapp_rw.contains("*Asigaye mu Ntoki / Amakonti*: 180000.00 RWF"));
    assert!(whatsapp_rw.contains("Bruno Bank: 100000.00 RWF"));
    assert!(whatsapp_rw.contains("Sister MoMo: 50000.00 RWF"));
    assert!(whatsapp_rw.contains("Roof Renovation: 250000.00 / 1000000.00 RWF"));

    let whatsapp_en = LocalizedExporter::format_mothers_report(&report, Locale::En);
    assert!(whatsapp_en.contains("Family Financial Summary (Mother's Report)"));
    assert!(whatsapp_en.contains("*Opening Balance*: 80000.00 RWF"));
    assert!(whatsapp_en.contains("*Total Income*: +200000.00 RWF"));
    assert!(whatsapp_en.contains("*Total Expenses*: -100000.00 RWF"));
    assert!(whatsapp_en.contains("*Closing Balance*: 180000.00 RWF"));
}
