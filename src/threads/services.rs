use std::sync::Arc;
use crate::threads::model::Thread;
use anyhow::Result;
use arc_swap::ArcSwap;
use axum::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ThreadsService {
    async fn create(&self) -> Result<Thread>;
    async fn list(&self) -> Vec<Thread>;
}

pub struct InMemoryThreadsService {
    //threads: Mutex<Vec<Thread>>,
    threads: ArcSwap<Vec<Thread>>,
}

impl InMemoryThreadsService {
    pub fn new() -> Self {
        //let threads = Mutex::new(Vec::new());
        let threads = ArcSwap::from_pointee(Vec::new());
        InMemoryThreadsService { threads }
    }
}

#[async_trait]
impl ThreadsService for InMemoryThreadsService {
    async fn create(&self) -> Result<Thread> {
        let thread = Thread { id: Uuid::new_v4() };
        let mut threads = self.threads.load().to_vec();
        threads.push(thread.clone());
        self.threads.store(Arc::new(threads));
        Ok(thread)
    }

    async fn list(&self) -> Vec<Thread> {
        //let threads = self.threads.lock().await;
        self.threads.load().to_vec()
    }
}
