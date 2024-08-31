mod config;
mod model;
mod router;
mod schema;
mod threads;

use crate::config::Settings;
use crate::router::ApplicationState;
use anyhow::Result;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use http::Method;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::timeout::TimeoutLayer;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_path("/Users/tim.van.wassenhove/dotenvs/openai.env")?;
    dotenv::from_path("/Users/tim.van.wassenhove/dotenvs/jaeger.env")?;

    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let settings = Settings::default();
    let state = Arc::new(ApplicationState::new(&settings)?);

    // build our application with a route
    let app = router::build_router(state)
        // include trace context as header into the response
        .layer(OtelInResponseLayer::default())
        //start OpenTelemetry trace on incoming request
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    //opentelemetry::global::shutdown_tracer_provider();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
