use crate::threads::model::Thread;
use arc_swap::ArcSwap;
use axum::async_trait;
use std::sync::Arc;
use uuid::{NoContext, Timestamp, Uuid};

#[async_trait]
pub trait ThreadsService: Send + Sync {
    async fn create(&self) -> Thread;
    async fn list(&self) -> Vec<Thread>;
}

pub struct MemorySessionsService {
    ts: Timestamp,
    sessions: ArcSwap<Vec<Thread>>,
}

impl MemorySessionsService {
    pub fn new() -> Self {
        let ts = Timestamp::from_unix(NoContext, 1497624119, 1234);
        let sessions = ArcSwap::new(Arc::new(Vec::new()));
        MemorySessionsService { ts, sessions }
    }
}

#[async_trait]
impl ThreadsService for MemorySessionsService {
    async fn create(&self) -> Thread {
        let thread = Thread {
            id: Uuid::new_v7(self.ts),
        };
        //self.sessions.rcu(|mut inner| *inner.push(thread.clone()));
        thread
    }

    async fn list(&self) -> Vec<Thread> {
        //self.threads.load().clone()
        todo!()
    }
}
