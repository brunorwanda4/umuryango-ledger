use chrono::{DateTime, Utc};
use fluent_bundle::{FluentArgs, FluentValue};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RealtimeEvent {
    IncomeAdded {
        family_id: Uuid,
        wallet_id: Uuid,
        wallet_name: String,
        amount: Decimal,
        currency: String,
        added_by_name: String,
        source_description: String,
        new_wallet_balance: Decimal,
        occurred_at: DateTime<Utc>,
    },
    ExpenseLogged {
        family_id: Uuid,
        wallet_id: Uuid,
        wallet_name: String,
        amount: Decimal,
        currency: String,
        logged_by_name: String,
        category: String,
        new_wallet_balance: Decimal,
        occurred_at: DateTime<Utc>,
    },
    HandoverCompleted {
        family_id: Uuid,
        from_wallet_name: String,
        to_wallet_name: String,
        amount: Decimal,
        currency: String,
        initiated_by_name: String,
        occurred_at: DateTime<Utc>,
    },
    ProjectOrLoanUpdated {
        family_id: Uuid,
        entity_type: String, // "project" or "loan"
        title: String,
    },
    AppUpdateAvailable {
        version: String,
        platform: String,
        is_mandatory: bool,
    },
}

impl RealtimeEvent {
    pub fn family_id(&self) -> Option<Uuid> {
        match self {
            RealtimeEvent::IncomeAdded { family_id, .. } => Some(*family_id),
            RealtimeEvent::ExpenseLogged { family_id, .. } => Some(*family_id),
            RealtimeEvent::HandoverCompleted { family_id, .. } => Some(*family_id),
            RealtimeEvent::ProjectOrLoanUpdated { family_id, .. } => Some(*family_id),
            RealtimeEvent::AppUpdateAvailable { .. } => None,
        }
    }

    pub fn format_notification(&self, locale: Locale) -> String {
        let mut args = FluentArgs::new();
        match self {
            RealtimeEvent::IncomeAdded {
                wallet_name,
                amount,
                added_by_name,
                source_description,
                ..
            } => {
                args.set("actor", FluentValue::from(added_by_name.as_str()));
                args.set("amount", FluentValue::from(amount.to_string()));
                args.set("wallet", FluentValue::from(wallet_name.as_str()));
                args.set("source", FluentValue::from(source_description.as_str()));
                Translator::t(locale, terms::NOTIF_INCOME_ADDED, Some(&args))
            }
            RealtimeEvent::ExpenseLogged {
                wallet_name,
                amount,
                logged_by_name,
                category,
                ..
            } => {
                args.set("actor", FluentValue::from(logged_by_name.as_str()));
                args.set("amount", FluentValue::from(amount.to_string()));
                args.set("wallet", FluentValue::from(wallet_name.as_str()));
                args.set("category", FluentValue::from(category.as_str()));
                Translator::t(locale, terms::NOTIF_EXPENSE_LOGGED, Some(&args))
            }
            RealtimeEvent::HandoverCompleted {
                from_wallet_name,
                to_wallet_name,
                amount,
                initiated_by_name,
                ..
            } => {
                args.set("actor", FluentValue::from(initiated_by_name.as_str()));
                args.set("amount", FluentValue::from(amount.to_string()));
                args.set("from_wallet", FluentValue::from(from_wallet_name.as_str()));
                args.set("to_wallet", FluentValue::from(to_wallet_name.as_str()));
                Translator::t(locale, terms::NOTIF_HANDOVER_COMPLETED, Some(&args))
            }
            RealtimeEvent::ProjectOrLoanUpdated {
                entity_type, title, ..
            } => {
                args.set("entity_type", FluentValue::from(entity_type.as_str()));
                args.set("title", FluentValue::from(title.as_str()));
                Translator::t(locale, terms::NOTIF_PROJECT_LOAN_UPDATED, Some(&args))
            }
            RealtimeEvent::AppUpdateAvailable {
                version,
                is_mandatory,
                ..
            } => {
                args.set("version", FluentValue::from(version.as_str()));
                let term = if *is_mandatory {
                    terms::NOTIF_APP_UPDATE_MANDATORY
                } else {
                    terms::NOTIF_APP_UPDATE_AVAILABLE
                };
                Translator::t(locale, term, Some(&args))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_realtime_notifications_bilingual() {
        let event = RealtimeEvent::IncomeAdded {
            family_id: Uuid::new_v4(),
            wallet_id: Uuid::new_v4(),
            wallet_name: "MoMo Main".to_string(),
            amount: Decimal::new(50000, 0),
            currency: "RWF".to_string(),
            added_by_name: "Keza".to_string(),
            source_description: "Agribusiness".to_string(),
            new_wallet_balance: Decimal::new(150000, 0),
            occurred_at: Utc::now(),
        };

        let en_str = event.format_notification(Locale::En);
        assert_eq!(en_str, "Keza added 50000 RWF to MoMo Main (Agribusiness).");

        let rw_str = event.format_notification(Locale::Rw);
        assert_eq!(
            rw_str,
            "Keza yongeye 50000 RWF kuri konti 'MoMo Main' (Agribusiness)."
        );

        let update_event = RealtimeEvent::AppUpdateAvailable {
            version: "0.2.0".to_string(),
            platform: "WINDOWS".to_string(),
            is_mandatory: false,
        };
        assert!(
            update_event
                .format_notification(Locale::En)
                .contains("0.2.0")
        );
        assert!(
            update_event
                .format_notification(Locale::Rw)
                .contains("0.2.0")
        );
    }
}
