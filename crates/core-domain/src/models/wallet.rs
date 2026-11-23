use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletChannel {
    Bank,
    MobileMoney,
    CashInHand,
}

impl WalletChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bank => "bank",
            Self::MobileMoney => "mobile_money",
            Self::CashInHand => "cash_in_hand",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Uuid,
    pub scope: LedgerScope,
    pub channel: WalletChannel,
    pub name: String,
    pub account_number: Option<String>,
    pub currency: String,
    pub balance: Decimal,
    pub custodian_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
