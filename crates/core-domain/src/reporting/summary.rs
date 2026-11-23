use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::models::{
    FuturePlan, Loan, Project, Transaction, TransactionType, Wallet, WalletChannel,
};
use crate::scope::LedgerScope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustodianWalletSummary {
    pub custodian_id: Option<Uuid>,
    pub wallet_name: String,
    pub channel: WalletChannel,
    pub balance: Decimal,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MothersReport {
    pub scope: LedgerScope,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub opening_balance: Decimal,
    pub total_income: Decimal,
    pub total_expenses: Decimal,
    pub closing_balance: Decimal,
    pub expenses_by_category: BTreeMap<String, Decimal>,
    pub custodian_balances: Vec<CustodianWalletSummary>,
    pub active_projects: Vec<Project>,
    pub open_loans: Vec<Loan>,
    pub future_plans: Vec<FuturePlan>,
}

pub fn generate_mothers_report(
    scope: LedgerScope,
    wallets: &[Wallet],
    transactions: &[Transaction],
    projects: &[Project],
    loans: &[Loan],
    plans: &[FuturePlan],
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
) -> MothersReport {
    // 1. Calculate closing balance from active wallets in this scope
    let mut closing_balance = Decimal::ZERO;
    let mut custodian_balances = Vec::new();

    for w in wallets {
        if w.scope == scope && w.is_active {
            closing_balance += w.balance;
            custodian_balances.push(CustodianWalletSummary {
                custodian_id: w.custodian_id,
                wallet_name: w.name.clone(),
                channel: w.channel,
                balance: w.balance,
                currency: w.currency.clone(),
            });
        }
    }

    // 2. Compute income, expenses (EXCLUDING handovers), and categorize
    let mut total_income = Decimal::ZERO;
    let mut total_expenses = Decimal::ZERO;
    let mut expenses_by_category: BTreeMap<String, Decimal> = BTreeMap::new();

    for tx in transactions {
        if tx.scope != scope {
            continue;
        }

        if tx.tx_date >= period_start && tx.tx_date <= period_end {
            match tx.tx_type {
                TransactionType::Income => {
                    total_income += tx.amount;
                }
                TransactionType::Expense => {
                    total_expenses += tx.amount;
                    *expenses_by_category
                        .entry(tx.category.clone())
                        .or_insert(Decimal::ZERO) += tx.amount;
                }
                TransactionType::Handover => {
                    // Internal custodian handovers are explicitly EXCLUDED from income/expense
                    // so internal transfers between family members never double-count as spending!
                }
            }
        }
    }

    // 3. Opening balance = closing - income + expenses
    let opening_balance = closing_balance - total_income + total_expenses;

    // 4. Filter active projects (if family scope)
    let active_projects = if let LedgerScope::Family { family_id } = scope {
        projects
            .iter()
            .filter(|p| p.family_id == family_id && p.status.is_active())
            .cloned()
            .collect()
    } else {
        Vec::new()
    };

    // 5. Filter active loans
    let open_loans = loans
        .iter()
        .filter(|l| l.scope == scope && l.status.is_active())
        .cloned()
        .collect();

    // 6. Filter active future plans
    let future_plans = plans
        .iter()
        .filter(|p| p.scope == scope && !p.is_achieved)
        .cloned()
        .collect();

    MothersReport {
        scope,
        period_start,
        period_end,
        opening_balance,
        total_income,
        total_expenses,
        closing_balance,
        expenses_by_category,
        custodian_balances,
        active_projects,
        open_loans,
        future_plans,
    }
}
