use chrono::{DateTime, Utc};
use rusqlite::{Result, params};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use umuryango_core_domain::models::TransactionType;
use uuid::Uuid;

use crate::db::LocalDatabase;

pub use umuryango_core_domain::SyncStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "data")]
pub enum SyncMutation {
    CreateTransaction {
        idempotency_key: Uuid,
        scope: String, // "personal" or "family"
        family_id: Option<Uuid>,
        source_wallet_id: Uuid,
        destination_wallet_id: Option<Uuid>,
        tx_type: TransactionType,
        amount: Decimal,
        category: String,
        description: Option<String>,
        paid_to: Option<String>,
        receipt_url: Option<String>,
        tx_date: Option<DateTime<Utc>>,
    },
    CreateHandover {
        idempotency_key: Uuid,
        family_id: Uuid,
        source_wallet_id: Uuid,
        destination_wallet_id: Uuid,
        amount: Decimal,
        notes: Option<String>,
    },
    RepayLoan {
        idempotency_key: Uuid,
        loan_id: Uuid,
        amount: Decimal,
    },
    AllocatePlan {
        idempotency_key: Uuid,
        plan_id: Uuid,
        amount: Decimal,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedMutation {
    pub id: Uuid,
    pub action: String,
    pub mutation: SyncMutation,
    pub status: String, // "pending", "in_flight", "synced", "failed"
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SyncQueueRepository<'a> {
    db: &'a LocalDatabase,
}

impl<'a> SyncQueueRepository<'a> {
    pub fn new(db: &'a LocalDatabase) -> Self {
        Self { db }
    }

    pub fn enqueue(&self, mutation: &SyncMutation) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let action_name = match mutation {
            SyncMutation::CreateTransaction { .. } => "create_transaction",
            SyncMutation::CreateHandover { .. } => "create_handover",
            SyncMutation::RepayLoan { .. } => "repay_loan",
            SyncMutation::AllocatePlan { .. } => "allocate_plan",
        };
        let payload = serde_json::to_string(mutation).unwrap_or_default();
        let now = Utc::now().to_rfc3339();

        self.db.conn().execute(
            r#"
            INSERT INTO sync_queue (id, action, payload, status, error_message, retry_count, created_at, updated_at)
            VALUES (?1, ?2, ?3, 'pending', NULL, 0, ?4, ?4)
            "#,
            params![id.to_string(), action_name, payload, now],
        )?;

        Ok(id)
    }

    pub fn get(&self, id: Uuid) -> Result<Option<QueuedMutation>> {
        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, action, payload, status, error_message, retry_count, created_at, updated_at
            FROM sync_queue
            WHERE id = ?1
            "#,
        )?;

        let mut rows = stmt.query_map([id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            let action: String = row.get(1)?;
            let payload: String = row.get(2)?;
            let status: String = row.get(3)?;
            let error_message: Option<String> = row.get(4)?;
            let retry_count: i32 = row.get(5)?;
            let created_str: String = row.get(6)?;
            let updated_str: String = row.get(7)?;

            let mutation: SyncMutation = serde_json::from_str(&payload).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

            Ok(QueuedMutation {
                id: Uuid::from_str(&id_str).unwrap_or_default(),
                action,
                mutation,
                status,
                error_message,
                retry_count,
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        if let Some(first) = rows.next() {
            Ok(Some(first?))
        } else {
            Ok(None)
        }
    }

    pub fn list_pending(&self, limit: usize) -> Result<Vec<QueuedMutation>> {
        let mut stmt = self.db.conn().prepare(
            r#"
            SELECT id, action, payload, status, error_message, retry_count, created_at, updated_at
            FROM sync_queue
            WHERE status IN ('pending', 'failed') AND retry_count < 5
            ORDER BY created_at ASC
            LIMIT ?1
            "#,
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            let id_str: String = row.get(0)?;
            let action: String = row.get(1)?;
            let payload: String = row.get(2)?;
            let status: String = row.get(3)?;
            let error_message: Option<String> = row.get(4)?;
            let retry_count: i32 = row.get(5)?;
            let created_str: String = row.get(6)?;
            let updated_str: String = row.get(7)?;

            let mutation: SyncMutation = serde_json::from_str(&payload).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    2,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

            Ok(QueuedMutation {
                id: Uuid::from_str(&id_str).unwrap_or_default(),
                action,
                mutation,
                status,
                error_message,
                retry_count,
                created_at: DateTime::parse_from_rfc3339(&created_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        Ok(list)
    }

    pub fn mark_in_flight(&self, id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.conn().execute(
            "UPDATE sync_queue SET status = 'in_flight', updated_at = ?1 WHERE id = ?2",
            params![now, id.to_string()],
        )?;
        Ok(())
    }

    pub fn mark_synced(&self, id: Uuid) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.conn().execute(
            "UPDATE sync_queue SET status = 'synced', updated_at = ?1 WHERE id = ?2",
            params![now, id.to_string()],
        )?;
        Ok(())
    }

    pub fn reset_to_pending(&self, id: Uuid, error: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.conn().execute(
            r#"
            UPDATE sync_queue
            SET status = 'pending',
                error_message = ?1,
                retry_count = retry_count + 1,
                updated_at = ?2
            WHERE id = ?3
            "#,
            params![error, now, id.to_string()],
        )?;
        Ok(())
    }

    pub fn mark_failed(&self, id: Uuid, error: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.db.conn().execute(
            r#"
            UPDATE sync_queue
            SET status = 'failed',
                error_message = ?1,
                retry_count = retry_count + 1,
                updated_at = ?2
            WHERE id = ?3
            "#,
            params![error, now, id.to_string()],
        )?;
        Ok(())
    }

    pub fn pending_count(&self) -> Result<usize> {
        let count: i64 = self.db.conn().query_row(
            "SELECT COUNT(*) FROM sync_queue WHERE status IN ('pending', 'in_flight')",
            [],
            |r| r.get(0),
        )?;
        Ok(count as usize)
    }

    pub fn sync_status(&self, is_online: bool) -> Result<SyncStatus> {
        if !is_online {
            return Ok(SyncStatus::Offline);
        }
        let count = self.pending_count()?;
        if count == 0 {
            Ok(SyncStatus::Synced)
        } else {
            Ok(SyncStatus::PendingSync(count))
        }
    }
}
