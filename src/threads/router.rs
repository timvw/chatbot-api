use crate::model::AppResult;
use crate::router::ApplicationState;
use crate::threads::schema::*;
use crate::threads::services::{InMemoryThreadsService, ThreadsService};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use std::sync::Arc;
use uuid::Uuid;

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
        .route("/:thread_id/messages", get(get_messages).post(add_message))
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

#[axum::debug_handler]
async fn get_messages(
    Path(thread_id): Path<Uuid>,
    State(ThreadsState {
        threads_service, ..
    }): State<ThreadsState>,
) -> AppResult<Json<Vec<Message>>> {
    let thread = threads_service.get(thread_id).await?;
    let messages = thread
        .messages
        .iter()
        .map(|x| Message { content: x.clone() })
        .collect();
    Ok(Json(messages))
}

#[axum::debug_handler]
async fn add_message(
    Path(thread_id): Path<Uuid>,
    State(ThreadsState {
        threads_service, ..
    }): State<ThreadsState>,
    Json(req): Json<AddMessageRequest>,
) -> AppResult<String> {
    let _ = threads_service.add_message(thread_id, &req.content).await?;
    /*Ok(Message {
        content: "OK".to_string(),
    })*/
    Ok("OK".to_string())
}
