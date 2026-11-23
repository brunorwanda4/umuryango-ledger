pub mod network_sim;

pub use network_sim::{FaultyNetworkProxy, NetworkMode};

use actix_web::{App, HttpServer, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use umuryango_api_server::middleware::create_jwt;
use umuryango_api_server::{AppState, FamilyHub, configure_app};
use umuryango_i18n::Locale;
use umuryango_storage_local::db::LocalDatabase;
use uuid::Uuid;

#[allow(dead_code)]
pub struct SeededUser {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub token: String,
}

#[allow(dead_code)]
pub struct SeededWallet {
    pub id: Uuid,
    pub name: String,
    pub custodian_id: Uuid,
    pub initial_balance: Decimal,
}

#[allow(dead_code)]
pub struct TestFixture {
    pub family_id: Uuid,
    pub mother: SeededUser,
    pub bruno: SeededUser,
    pub bruno_wallet: SeededWallet,
    pub brother: SeededUser,
    pub brother_wallet: SeededWallet,
}

pub struct TestServer {
    pub pool: PgPool,
    pub family_hub: Arc<FamilyHub>,
    pub port: u16,
    pub base_url: String,
    pub ws_url: String,
    pub jwt_secret: String,
}

impl TestServer {
    pub async fn start() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:0004@localhost:5432/umuryango_ledger".to_string()
        });

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            "test_super_secret_jwt_key_min_32_chars_long_umuryango".to_string()
        });

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test PostgreSQL database");

        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .expect("Failed to apply database migrations");

        let family_hub = Arc::new(FamilyHub::new());
        let app_state = AppState::with_hub(
            pool.clone(),
            jwt_secret.clone(),
            "http://localhost:3000".to_string(),
            family_hub.clone(),
        );

        let tcp_listener =
            std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind ephemeral test port");
        let port = tcp_listener
            .local_addr()
            .expect("Failed to get local addr")
            .port();

        let state_data = app_state.clone();
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state_data.clone()))
                .configure(configure_app)
        })
        .listen(tcp_listener)
        .expect("Failed to listen on tcp listener")
        .run();

        tokio::spawn(server);

        let base_url = format!("http://127.0.0.1:{}", port);
        let ws_url = format!("ws://127.0.0.1:{}/api/v1/ws", port);

        Self {
            pool,
            family_hub,
            port,
            base_url,
            ws_url,
            jwt_secret,
        }
    }

    pub async fn clean_db(&self) {
        sqlx::query(
            r#"
            TRUNCATE TABLE
                transactions,
                custody_handovers,
                project_allocations,
                loans,
                projects,
                future_plans,
                wallets,
                family_members,
                zero_login_reports,
                families,
                users
            CASCADE
            "#,
        )
        .execute(&self.pool)
        .await
        .ok();
    }

    pub async fn seed_test_family(&self) -> TestFixture {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(b"TestPass123!", &salt)
            .expect("Failed to hash password")
            .to_string();

        let now = Utc::now();

        // 1. Mother (Head)
        let mother_id = Uuid::new_v4();
        let mother_email = format!("mother_{}@test.rw", mother_id.as_simple());
        sqlx::query(
            r#"
            INSERT INTO users (id, email, full_name, password_hash, preferred_locale, is_active, created_at, updated_at)
            VALUES ($1, $2, 'Mama Keza', $3, 'rw', TRUE, $4, $4)
            "#,
        )
        .bind(mother_id)
        .bind(&mother_email)
        .bind(&password_hash)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert mother user");

        let mother_token =
            create_jwt(mother_id, Locale::Rw, &self.jwt_secret, 72).expect("JWT error");

        // 2. Family
        let family_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO families (id, name, description, currency, created_by, created_at, updated_at)
            VALUES ($1, 'Umuryango Keza', 'Test Family Ledger', 'RWF', $2, $3, $3)
            "#,
        )
        .bind(family_id)
        .bind(mother_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert family");

        // Mother membership (Head)
        sqlx::query(
            r#"
            INSERT INTO family_members (id, family_id, user_id, role, is_active, joined_at, updated_at)
            VALUES ($1, $2, $3, 'head', TRUE, $4, $4)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(family_id)
        .bind(mother_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert mother membership");

        // 3. Bruno (Custodian - Bank)
        let bruno_id = Uuid::new_v4();
        let bruno_email = format!("bruno_{}@test.rw", bruno_id.as_simple());
        sqlx::query(
            r#"
            INSERT INTO users (id, email, full_name, password_hash, preferred_locale, is_active, created_at, updated_at)
            VALUES ($1, $2, 'Bruno (Bank Custodian)', $3, 'en', TRUE, $4, $4)
            "#,
        )
        .bind(bruno_id)
        .bind(&bruno_email)
        .bind(&password_hash)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert bruno user");

        let bruno_token =
            create_jwt(bruno_id, Locale::En, &self.jwt_secret, 72).expect("JWT error");

        sqlx::query(
            r#"
            INSERT INTO family_members (id, family_id, user_id, role, is_active, joined_at, updated_at)
            VALUES ($1, $2, $3, 'treasurer', TRUE, $4, $4)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(family_id)
        .bind(bruno_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert bruno membership");

        let bruno_wallet_id = Uuid::new_v4();
        let bruno_init_balance = Decimal::new(150000, 0);
        sqlx::query(
            r#"
            INSERT INTO wallets (id, scope, family_id, wallet_type, name, currency, balance, custodian_id, is_active, created_at, updated_at)
            VALUES ($1, 'family', $2, 'bank', 'BK Family Custody', 'RWF', $3, $4, TRUE, $5, $5)
            "#,
        )
        .bind(bruno_wallet_id)
        .bind(family_id)
        .bind(bruno_init_balance)
        .bind(bruno_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert bruno wallet");

        // 4. Brother (Custodian - Cash)
        let brother_id = Uuid::new_v4();
        let brother_email = format!("brother_{}@test.rw", brother_id.as_simple());
        sqlx::query(
            r#"
            INSERT INTO users (id, email, full_name, password_hash, preferred_locale, is_active, created_at, updated_at)
            VALUES ($1, $2, 'Eric (Cash Custodian)', $3, 'rw', TRUE, $4, $4)
            "#,
        )
        .bind(brother_id)
        .bind(&brother_email)
        .bind(&password_hash)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert brother user");

        let brother_token =
            create_jwt(brother_id, Locale::Rw, &self.jwt_secret, 72).expect("JWT error");

        sqlx::query(
            r#"
            INSERT INTO family_members (id, family_id, user_id, role, is_active, joined_at, updated_at)
            VALUES ($1, $2, $3, 'treasurer', TRUE, $4, $4)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(family_id)
        .bind(brother_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert brother membership");

        let brother_wallet_id = Uuid::new_v4();
        let brother_init_balance = Decimal::new(50000, 0);
        sqlx::query(
            r#"
            INSERT INTO wallets (id, scope, family_id, wallet_type, name, currency, balance, custodian_id, is_active, created_at, updated_at)
            VALUES ($1, 'family', $2, 'cash', 'Home Cash Box', 'RWF', $3, $4, TRUE, $5, $5)
            "#,
        )
        .bind(brother_wallet_id)
        .bind(family_id)
        .bind(brother_init_balance)
        .bind(brother_id)
        .bind(now)
        .execute(&self.pool)
        .await
        .expect("Failed to insert brother wallet");

        TestFixture {
            family_id,
            mother: SeededUser {
                id: mother_id,
                email: mother_email,
                full_name: "Mama Keza".to_string(),
                token: mother_token,
            },
            bruno: SeededUser {
                id: bruno_id,
                email: bruno_email,
                full_name: "Bruno (Bank Custodian)".to_string(),
                token: bruno_token,
            },
            bruno_wallet: SeededWallet {
                id: bruno_wallet_id,
                name: "BK Family Custody".to_string(),
                custodian_id: bruno_id,
                initial_balance: bruno_init_balance,
            },
            brother: SeededUser {
                id: brother_id,
                email: brother_email,
                full_name: "Eric (Cash Custodian)".to_string(),
                token: brother_token,
            },
            brother_wallet: SeededWallet {
                id: brother_wallet_id,
                name: "Home Cash Box".to_string(),
                custodian_id: brother_id,
                initial_balance: brother_init_balance,
            },
        }
    }

    pub fn create_isolated_local_db(
        &self,
    ) -> (tempfile::TempDir, Arc<std::sync::Mutex<LocalDatabase>>) {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let db_path = temp_dir.path().join("umuryango_test.db");
        let db = LocalDatabase::open(&db_path).expect("Failed to open SQLite test database");
        (temp_dir, Arc::new(std::sync::Mutex::new(db)))
    }
}
