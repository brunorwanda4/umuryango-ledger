use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_actix_web::TracingLogger;
use umuryango_api_server::{AppState, configure_app};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "4433".to_string())
        .parse::<u16>()
        .unwrap_or(4433);

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
        "[Umuryango Ledger] API server starting on http://{}:{}",
        host, port
    );
    println!(
        "[Umuryango Ledger] Swagger UI documentation available at http://{}:{}/swagger-ui/",
        host, port
    );

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_state.clone()))
            .configure(configure_app)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
