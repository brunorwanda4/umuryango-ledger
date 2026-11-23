use rust_decimal::Decimal;

use crate::error::DomainError;
use crate::models::{FamilyRole, Transaction, TransactionType, Wallet};
use crate::scope::LedgerScope;

pub struct LedgerValidator;

impl LedgerValidator {
    pub fn validate_and_apply_transaction(
        tx: &Transaction,
        actor_role: FamilyRole,
        wallets: &mut [Wallet],
    ) -> Result<(), DomainError> {
        // 1. Transaction amount must be strictly greater than zero
        if tx.amount <= Decimal::ZERO {
            return Err(DomainError::InvalidAmount);
        }

        // 2. Enforce family role permissions
        if tx.scope.is_family()
            && matches!(
                tx.tx_type,
                TransactionType::Expense | TransactionType::Handover
            )
            && actor_role == FamilyRole::Member
        {
            return Err(DomainError::UnauthorizedRole);
        }

        match tx.tx_type {
            TransactionType::Income => {
                let src_idx = wallets
                    .iter()
                    .position(|w| w.id == tx.source_wallet_id)
                    .ok_or(DomainError::NotFound)?;

                if wallets[src_idx].scope != tx.scope {
                    return Err(DomainError::ScopeMismatch);
                }

                wallets[src_idx].balance += tx.amount;
            }

            TransactionType::Expense => {
                let src_idx = wallets
                    .iter()
                    .position(|w| w.id == tx.source_wallet_id)
                    .ok_or(DomainError::NotFound)?;

                if wallets[src_idx].scope != tx.scope {
                    return Err(DomainError::ScopeMismatch);
                }

                if wallets[src_idx].balance < tx.amount {
                    return Err(DomainError::InsufficientCustodyBalance {
                        wallet: wallets[src_idx].name.clone(),
                        available: wallets[src_idx].balance,
                        requested: tx.amount,
                    });
                }

                wallets[src_idx].balance -= tx.amount;
            }

            TransactionType::Handover => {
                let dest_id = tx
                    .destination_wallet_id
                    .ok_or(DomainError::InvalidHandover)?;

                if tx.source_wallet_id == dest_id {
                    return Err(DomainError::InvalidHandover);
                }

                let src_idx = wallets
                    .iter()
                    .position(|w| w.id == tx.source_wallet_id)
                    .ok_or(DomainError::NotFound)?;

                let tgt_idx = wallets
                    .iter()
                    .position(|w| w.id == dest_id)
                    .ok_or(DomainError::NotFound)?;

                // Both wallets must belong to LedgerScope::Family with the same family_id
                match (wallets[src_idx].scope, wallets[tgt_idx].scope) {
                    (
                        LedgerScope::Family { family_id: f1 },
                        LedgerScope::Family { family_id: f2 },
                    ) if f1 == f2 => {
                        if tx.scope != wallets[src_idx].scope {
                            return Err(DomainError::ScopeMismatch);
                        }
                    }
                    _ => return Err(DomainError::InvalidHandover),
                }

                // Check custody balance of source
                if wallets[src_idx].balance < tx.amount {
                    return Err(DomainError::InsufficientCustodyBalance {
                        wallet: wallets[src_idx].name.clone(),
                        available: wallets[src_idx].balance,
                        requested: tx.amount,
                    });
                }

                // Deduct from source and add to destination without counting as an expense
                wallets[src_idx].balance -= tx.amount;
                wallets[tgt_idx].balance += tx.amount;
            }
        }

        Ok(())
    }
}
