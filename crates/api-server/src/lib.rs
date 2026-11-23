use actix_web::{HttpResponse, Responder, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod error;
pub mod handlers;
pub mod middleware;
pub mod openapi;
pub mod realtime;
pub mod state;

pub use error::ApiError;
pub use openapi::ApiDoc;
pub use realtime::{FamilyHub, RealtimeEvent};
pub use state::AppState;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "umuryango-api-server",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
    .route("/health", web::get().to(health_check))
    .service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_check))
            .route("/ws", web::get().to(realtime::ws_handler))
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/locale", web::patch().to(handlers::auth::update_locale)),
            )
            .service(
                web::scope("/families")
                    .route("", web::get().to(handlers::families::list_families))
                    .route("", web::post().to(handlers::families::create_family))
                    .route(
                        "/pending-invites",
                        web::get().to(handlers::families::list_pending_invites),
                    )
                    .route("/join", web::post().to(handlers::families::join_family))
                    .route(
                        "/{family_id}/members/{member_id}/role",
                        web::patch().to(handlers::families::update_member_role),
                    ),
            )
            .service(
                web::scope("/wallets")
                    .route("", web::get().to(handlers::wallets::list_wallets))
                    .route("", web::post().to(handlers::wallets::create_wallet)),
            )
            .service(
                web::scope("/transactions")
                    .route("", web::get().to(handlers::transactions::list_transactions))
                    .route(
                        "",
                        web::post().to(handlers::transactions::create_transaction),
                    ),
            )
            .service(
                web::scope("/projects")
                    .route("", web::get().to(handlers::projects::list_projects))
                    .route("", web::post().to(handlers::projects::create_project)),
            )
            .service(
                web::scope("/loans")
                    .route("", web::get().to(handlers::projects::list_loans))
                    .route("", web::post().to(handlers::projects::create_loan))
                    .route(
                        "/{id}/repay",
                        web::post().to(handlers::projects::repay_loan),
                    ),
            )
            .service(
                web::scope("/plans")
                    .route("", web::get().to(handlers::projects::list_plans))
                    .route("", web::post().to(handlers::projects::create_plan))
                    .route(
                        "/{id}/allocate",
                        web::post().to(handlers::projects::allocate_plan),
                    ),
            )
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
                    )
                    .route(
                        "/shared/{token}",
                        web::get().to(handlers::reports::get_public_report),
                    ),
            )
            .service(
                web::scope("/releases")
                    .route("/check", web::get().to(handlers::releases::check_release))
                    .route(
                        "/publish",
                        web::post().to(handlers::releases::publish_release),
                    ),
            )
            .service(
                web::scope("/ota")
                    .route("/bundle", web::get().to(handlers::releases::get_ota_bundle))
                    .route(
                        "/publish",
                        web::post().to(handlers::releases::publish_ota_bundle),
                    ),
            ),
    );
}
