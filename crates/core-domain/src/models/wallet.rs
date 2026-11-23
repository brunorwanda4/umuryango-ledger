use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = "bank")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Wallet {
    #[schema(example = "6cb3b9a5-1d42-43bb-a5a1-778912efc401")]
    pub id: Uuid,
    pub scope: LedgerScope,
    pub channel: WalletChannel,
    #[schema(example = "MTN MoMo Custody")]
    pub name: String,
    #[schema(example = "+250788123456")]
    pub account_number: Option<String>,
    #[schema(example = "RWF")]
    pub currency: String,
    #[schema(example = "250000.00")]
    pub balance: Decimal,
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub custodian_id: Option<Uuid>,
    #[schema(example = true)]
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
