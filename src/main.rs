mod sessions;

use anyhow::Result;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // build our application with a route
    let app = build_router().layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn build_router() -> Router {
    Router::new().nest("/sessions", sessions::router::build_router())
}
