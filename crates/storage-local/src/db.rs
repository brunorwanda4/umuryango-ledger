use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};

pub struct LocalDatabase {
    conn: Connection,
}

impl LocalDatabase {
    /// Opens an in-memory SQLite database (ideal for tests and temporary sessions).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Opens or creates the SQLite database at the specified file path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Opens or creates the default offline database (`umuryango_offline.db`).
    pub fn open_default() -> Result<Self> {
        Self::open(Self::default_db_path())
    }

    /// Returns the standard path for the local SQLite database.
    pub fn default_db_path() -> PathBuf {
        PathBuf::from("umuryango_offline.db")
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;

            -- Cached Wallets (both Personal and Family Custody)
            CREATE TABLE IF NOT EXISTS wallets (
                id TEXT PRIMARY KEY,
                scope_type TEXT NOT NULL, -- 'personal' or 'family'
                family_id TEXT,
                user_id TEXT,
                channel TEXT NOT NULL, -- 'bank', 'momo', 'cash'
                name TEXT NOT NULL,
                account_number TEXT,
                currency TEXT NOT NULL DEFAULT 'RWF',
                balance TEXT NOT NULL DEFAULT '0',
                custodian_id TEXT,
                is_active INTEGER NOT NULL DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_wallets_scope ON wallets(scope_type, family_id, user_id);

            -- Cached Transactions & Handovers
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                scope_type TEXT NOT NULL,
                family_id TEXT,
                user_id TEXT NOT NULL,
                source_wallet_id TEXT NOT NULL,
                destination_wallet_id TEXT,
                tx_type TEXT NOT NULL, -- 'income', 'expense', 'handover'
                amount TEXT NOT NULL,
                currency TEXT NOT NULL DEFAULT 'RWF',
                category TEXT NOT NULL,
                description TEXT,
                authorized_by TEXT NOT NULL,
                paid_to TEXT,
                receipt_url TEXT,
                tx_date TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_transactions_scope ON transactions(scope_type, family_id, user_id);
            CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(tx_date DESC);

            -- Cached Projects
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                target_amount TEXT NOT NULL,
                allocated_amount TEXT NOT NULL DEFAULT '0',
                status TEXT NOT NULL DEFAULT 'planning',
                start_date TEXT,
                target_end_date TEXT,
                created_by TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_projects_family ON projects(family_id);

            -- Cached Loans
            CREATE TABLE IF NOT EXISTS loans (
                id TEXT PRIMARY KEY,
                scope_type TEXT NOT NULL,
                family_id TEXT,
                user_id TEXT NOT NULL,
                counterparty_name TEXT NOT NULL,
                principal_amount TEXT NOT NULL,
                balance_remaining TEXT NOT NULL,
                interest_rate TEXT NOT NULL DEFAULT '0',
                due_date TEXT,
                direction TEXT NOT NULL, -- 'lent', 'borrowed'
                status TEXT NOT NULL DEFAULT 'active',
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_loans_scope ON loans(scope_type, family_id, user_id);

            -- Cached Future Plans
            CREATE TABLE IF NOT EXISTS future_plans (
                id TEXT PRIMARY KEY,
                scope_type TEXT NOT NULL,
                family_id TEXT,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                estimated_cost TEXT NOT NULL,
                target_date TEXT,
                is_achieved INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_plans_scope ON future_plans(scope_type, family_id, user_id);

            -- User Theme Preferences
            CREATE TABLE IF NOT EXISTS user_theme_prefs (
                user_id TEXT PRIMARY KEY,
                theme_mode TEXT NOT NULL DEFAULT 'light',
                crimson_deep TEXT NOT NULL DEFAULT '#780000',
                crimson_bright TEXT NOT NULL DEFAULT '#c1121f',
                cream_warm TEXT NOT NULL DEFAULT '#fdf0d5',
                prussian_navy TEXT NOT NULL DEFAULT '#003049',
                steel_blue TEXT NOT NULL DEFAULT '#669bbc',
                updated_at TEXT NOT NULL
            );

            -- Sync Queue for offline mutations
            CREATE TABLE IF NOT EXISTS sync_queue (
                id TEXT PRIMARY KEY,
                action TEXT NOT NULL,
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                error_message TEXT,
                retry_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_sync_queue_status ON sync_queue(status, created_at ASC);
            "#,
        )?;
        Ok(())
    }
}
