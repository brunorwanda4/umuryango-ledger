use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_actix_web::TracingLogger;

mod error;
mod handlers;
mod middleware;
pub mod realtime;
mod state;

use state::AppState;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "umuryango-api-server",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/umuryango_ledger".to_string()
    });

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
        "default_super_secret_jwt_key_min_32_characters_long_for_umuryango".to_string()
    });

    let public_web_url =
        env::var("PUBLIC_WEB_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&database_url)
        .expect("Failed to initialize database pool configuration");

    let app_state = AppState::new(pool, jwt_secret, public_web_url);

    println!(
        "🇷🇼 Umuryango Ledger API server starting on http://{}:{}",
        host, port
    );

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_state.clone()))
            // Health check
            .route("/health", web::get().to(health_check))
            // API V1
            .service(
                web::scope("/api/v1")
                    // Health check sub-route
                    .route("/health", web::get().to(health_check))
                    // Real-time WebSocket
                    .route("/ws", web::get().to(realtime::ws_handler))
                    // Auth
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                            .route("/locale", web::patch().to(handlers::auth::update_locale)),
                    )
                    // Families
                    .service(
                        web::scope("/families")
                            .route("", web::get().to(handlers::families::list_families))
                            .route("", web::post().to(handlers::families::create_family))
                            .route("/join", web::post().to(handlers::families::join_family))
                            .route(
                                "/{family_id}/members/{member_id}/role",
                                web::patch().to(handlers::families::update_member_role),
                            ),
                    )
                    // Wallets
                    .service(
                        web::scope("/wallets")
                            .route("", web::get().to(handlers::wallets::list_wallets))
                            .route("", web::post().to(handlers::wallets::create_wallet)),
                    )
                    // Transactions
                    .service(
                        web::scope("/transactions")
                            .route("", web::get().to(handlers::transactions::list_transactions))
                            .route(
                                "",
                                web::post().to(handlers::transactions::create_transaction),
                            ),
                    )
                    // Projects
                    .service(
                        web::scope("/projects")
                            .route("", web::get().to(handlers::projects::list_projects))
                            .route("", web::post().to(handlers::projects::create_project)),
                    )
                    // Loans
                    .service(
                        web::scope("/loans")
                            .route("", web::get().to(handlers::projects::list_loans))
                            .route("", web::post().to(handlers::projects::create_loan))
                            .route(
                                "/{id}/repay",
                                web::post().to(handlers::projects::repay_loan),
                            ),
                    )
                    // Future Plans
                    .service(
                        web::scope("/plans")
                            .route("", web::get().to(handlers::projects::list_plans))
                            .route("", web::post().to(handlers::projects::create_plan))
                            .route(
                                "/{id}/allocate",
                                web::post().to(handlers::projects::allocate_plan),
                            ),
                    )
                    // Reports & Magic Links
                    .service(
                        web::scope("/reports")
                            .route(
                                "/summary",
                                web::get().to(handlers::reports::get_summary_report),
                            )
                            .route(
                                "/share",
                                web::post().to(handlers::reports::create_share_link),
                            )
                            .route(
                                "/public/{token}",
                                web::get().to(handlers::reports::get_public_report),
                            ),
                    ),
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
