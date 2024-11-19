#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use dotenv::dotenv;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use listoplate::app::*;
    use listoplate::fileserv::file_and_error_handler;

    // ingest env vars

    let _ = dotenv::dotenv();

    // setup logging

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info,sqlx=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // middleware

    let middleware_stack = ServiceBuilder::new().layer(
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
                    .level(tracing::Level::INFO)
                    .latency_unit(LatencyUnit::Millis),
            ),
    );

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(middleware_stack);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    use leptos::*;
    use listoplate::app::App;

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {  <App/> }
    });
}
