use crate::threads::model::Thread;
use anyhow::Result;
use axum::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait]
pub trait ThreadsService {
    async fn create(&self) -> Result<Thread>;
    async fn list(&self) -> Vec<Thread>;
}

pub struct InMemoryThreadsService {
    threads: Mutex<Vec<Thread>>,
}

impl InMemoryThreadsService {
    pub fn new() -> Self {
        let threads = Mutex::new(Vec::new());
        InMemoryThreadsService { threads }
    }
}

#[async_trait]
impl ThreadsService for InMemoryThreadsService {
    async fn create(&self) -> Result<Thread> {
        let thread = Thread { id: Uuid::new_v4() };
        let mut threads = self.threads.lock().await;
        threads.push(thread.clone());
        Ok(thread)
    }

    async fn list(&self) -> Vec<Thread> {
        let threads = self.threads.lock().await;
        threads.clone()
    }
}
