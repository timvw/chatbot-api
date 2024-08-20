use crate::model::AppResult;
use crate::router::ApplicationState;
use crate::threads::schema::Thread;
use crate::threads::services::{InMemoryThreadsService, ThreadsService};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;

#[derive(Clone)]
struct ThreadsState {
    //application_state: Arc<ApplicationState>,
    threads_service: Arc<dyn ThreadsService + Send + Sync>,
}

pub fn build_router(_: Arc<ApplicationState>) -> Router {
    let thread_state = ThreadsState {
        //application_state: state,
        threads_service: Arc::new(InMemoryThreadsService::new()),
    };

    Router::new()
        .route("/", get(get_threads).post(create_thread))
        .with_state(thread_state)
}

#[axum::debug_handler]
async fn get_threads(
    State(ThreadsState {
        threads_service, ..
    }): State<ThreadsState>,
) -> Json<Vec<Thread>> {
    let threads = threads_service.list().await;
    let model_threads = threads.iter().map(From::from).collect();
    Json(model_threads)
}

async fn create_thread(
    State(ThreadsState {
        threads_service, ..
    }): State<ThreadsState>,
) -> AppResult<Json<Thread>> {
    let thread = threads_service.create().await?;
    Ok(Json(thread.into()))
}
