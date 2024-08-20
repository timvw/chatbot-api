use crate::router::ApplicationState;
use crate::threads::schema::Thread;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;

pub fn build_router(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/", get(get_threads).post(create_thread))
        .with_state(state)
}

#[axum::debug_handler]
async fn get_threads() -> Json<Vec<Thread>> {
    todo!()
}

async fn create_thread(State(state): State<Arc<ApplicationState>>) -> Json<String> {
    //let model_thread = crate::threads::model::Thread::default();
    //Json(model_thread.into())
    let settings = state.settings.load();
    let log_level = settings
        .logging
        .log_level
        .clone()
        .unwrap_or("DEBUG".to_string());
    Json(log_level)
}
