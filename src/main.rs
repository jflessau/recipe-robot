#[cfg(feature = "ssr")]
use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    Surreal,
};
#[cfg(feature = "ssr")]
use surrealdb_migrations::MigrationRunner;

#[cfg(feature = "ssr")]
use anyhow::{Context, Result};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::body::Body;
    use axum::{
        extract::{Extension, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use axum_extra::extract::cookie::CookieJar;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use leptos::*;

    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use recipe_ranger::api::{AuthenticatedUser, Claims};
    use recipe_ranger::app::*;
    use recipe_ranger::fileserv::file_and_error_handler;
    use recipe_ranger::AppState;
    use tower::ServiceBuilder;
    use tower_http::{
        timeout::TimeoutLayer,
        trace::{DefaultOnResponse, TraceLayer},
        LatencyUnit,
    };
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // ingest env vars

    let _ = dotenv::dotenv();
    let jwt_secret = std::env::var("JWT_SECRET").expect("missing JWT_SECRET");

    // setup logging

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,sqlx=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let logging_middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                let mut request_info = format!("{} {}", request.method(), request.uri().path(),);

                if let Some(params) = request.uri().query() {
                    request_info.push_str(&format!("?{}", params));
                }

                tracing::info_span!("", request_info)
            })
            .on_response(
                DefaultOnResponse::new()
                    .level(tracing::Level::DEBUG)
                    .latency_unit(LatencyUnit::Millis),
            ),
    );

    // create app state

    let db = setup_db().await.unwrap_or_else(|err| {
        log::error!("💥 error in setting up db: {err:?}");
        std::process::exit(1);
    });

    let app_state = AppState {
        db,
        jwt_secret: jwt_secret.clone(),
    };
    let app_state_clone = app_state.clone();

    // auth middleware for server functions (api endpoints)

    async fn auth(
        Extension(app_state): Extension<AppState>,
        cookies: CookieJar,
        req: Request<Body>,
    ) -> impl IntoResponse {
        let username: Option<String> = cookies
            .get("jwt")
            .and_then(|cookie| {
                let jwt = cookie.value();
                decode::<Claims>(
                    jwt,
                    &DecodingKey::from_base64_secret(&app_state.jwt_secret).unwrap(),
                    &Validation::default(),
                )
                .ok()
            })
            .map(|data| data.claims.sub);

        handle_server_fns_with_context(
            move || {
                provide_context(username.clone().map(AuthenticatedUser::new));
                provide_context(app_state.clone());
            },
            req,
        )
        .await
    }

    // leptos routes handler for ssr content

    async fn leptos_routes_handler(
        State(state): State<LeptosOptions>,
        req: Request<Body>,
    ) -> Response {
        let handler =
            leptos_axum::render_app_to_stream_with_context(state, move || {}, || view! { <App /> });
        handler(req).await.into_response()
    }

    // setup router

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/*fn_name", get(auth).post(auth))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(logging_middleware)
        .layer(Extension(app_state_clone));

    // start server

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    use leptos::*;
    use recipe_ranger::app::App;

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    });
}
