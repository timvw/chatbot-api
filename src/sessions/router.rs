use axum::routing::get;
use axum::Router;

pub fn build_router() -> Router {
    Router::new().route("/", get(get_sessions))
}

async fn get_sessions() -> &'static str {
    "Hello, World!"
}
