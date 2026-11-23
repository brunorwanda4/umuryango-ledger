use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{Wallet, WalletChannel};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceBreakdown {
    pub total_bank: Decimal,
    pub total_momo: Decimal,
    pub total_cash: Decimal,
    pub total_assets: Decimal,
    pub by_custodian: HashMap<Uuid, Decimal>,
    pub by_wallet: HashMap<Uuid, Decimal>,
}

impl BalanceBreakdown {
    pub fn compute(wallets: &[Wallet]) -> Self {
        let mut breakdown = BalanceBreakdown::default();

        for w in wallets {
            if !w.is_active {
                continue;
            }

            match w.channel {
                WalletChannel::Bank => breakdown.total_bank += w.balance,
                WalletChannel::MobileMoney => breakdown.total_momo += w.balance,
                WalletChannel::CashInHand => breakdown.total_cash += w.balance,
            }

            breakdown.total_assets += w.balance;
            breakdown.by_wallet.insert(w.id, w.balance);

            if let Some(custodian) = w.custodian_id {
                *breakdown
                    .by_custodian
                    .entry(custodian)
                    .or_insert(Decimal::ZERO) += w.balance;
            }
        }

        breakdown
    }
}
