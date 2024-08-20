use crate::router::ApplicationState;
use crate::threads::schema::Thread;
use crate::threads::services::{InMemoryThreadsService, ThreadsService};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;
use crate::schema::AppError;

#[derive(Clone)]
pub struct ThreadsState {
    application_state: Arc<ApplicationState>,
    threads_service: Arc<dyn ThreadsService + Send + Sync>,
}

pub fn build_router(state: Arc<ApplicationState>) -> Router {
    let thread_state = ThreadsState {
        application_state: state,
        threads_service: Arc::new(InMemoryThreadsService::new()),
    };

    Router::new()
        .route("/", get(get_threads).post(create_thread))
        .with_state(thread_state.into())
}

#[axum::debug_handler]
async fn get_threads(State(state): State<Arc<ThreadsState>>) -> Json<Vec<Thread>> {
    let threads_service = state.threads_service.clone();
    let threads = threads_service.list().await;
    let ss = threads.iter().map(From::from).collect();
    Json(ss)
}

async fn create_thread(State(state): State<Arc<ThreadsState>>) -> Result<Json<Thread>, AppError> {
    //let model_thread = crate::threads::model::Thread::default();
    //Json(model_thread.into())
    let threads_service = state.threads_service.clone();
    let thread = threads_service.create().await?;
    Ok(Json(thread.into()))
}
