pub mod ai;
pub mod db;
pub mod error;
pub mod handler;
pub mod model;
pub mod prelude;
mod util;

use prelude::*;

use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub jwt_secret: String,
}

pub async fn app() -> error::Result<Router> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        .allow_headers(vec![
            ACCEPT,
            AUTHORIZATION,
            CONTENT_TYPE,
            COOKIE,
            USER_AGENT,
        ])
        .allow_credentials(true)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_origin(AllowOrigin::list(vec![env::var("CORS_ALLOWED_ORIGIN")
            .unwrap_or_else(|_| "http://localhost".to_owned())
            .parse()
            .expect("parsing CORS_ALLOWED_ORIGIN fails")]));

    let jwt_secret = std::env::var("JWT_SECRET").expect("missing JWT_SECRET");
    let db = setup_db().await.unwrap_or_else(|err| {
        error!("💽💥 error while setting up db: {err:?}");
        std::process::exit(1);
    });

    let app_state = AppState {
        db,
        jwt_secret: jwt_secret.clone(),
    };

    let middleware_stack = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let mut request_info =
                        format!("{} {}", request.method(), request.uri().path(),);

                    if let Some(params) = request.uri().query() {
                        request_info.push_str(&format!("?{}", params));
                    }

                    tracing::info_span!("", request_info)
                })
                .on_response(
                    DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(cors)
        .layer(Extension(app_state));

    let app = Router::new()
        .route("/health", get(health))
        .layer(middleware_stack);

    Ok(app)
}

async fn health() -> error::Result<StatusCode> {
    Ok(StatusCode::OK)
}

async fn setup_db() -> Result<Surreal<Any>> {
    let db_url = std::env::var("DB_URL").expect("DB_URL must be set");
    let db = connect(db_url).await.context("fails to connect to db")?;

    db.signin(Root {
        username: &std::env::var("DB_USER").expect("DB_USER must be set"),
        password: &std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
    })
    .await
    .context("fails to signin")?;

    db.use_ns("default")
        .use_db("default")
        .await
        .context("fails to set namespace")?;

    MigrationRunner::new(&db)
        .up()
        .await
        .expect("Failed to apply migrations");

    Ok(db)
}
