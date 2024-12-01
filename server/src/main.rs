use dotenv::dotenv;
use recipe_robot::app;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = app().await.expect("fails build server");

    let port = dotenv::var("PORT")
        .unwrap_or_else(|_| "1313".into())
        .parse::<u16>()
        .expect("invalid PORT");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("fails to bind listener");
    info!("listening on {}", addr);

    axum::serve(listener, app).await.expect("server fails");
}
