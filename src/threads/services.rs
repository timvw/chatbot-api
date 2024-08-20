use crate::threads::model::Thread;
use crate::threads::schema::Message;
use anyhow::{anyhow, Result};
use arc_swap::ArcSwap;
use axum::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::{uuid, Uuid};

#[async_trait]
pub trait ThreadsService {
    async fn create(&self) -> Result<Thread>;
    async fn list(&self) -> Vec<Thread>;
    async fn get(&self, id: Uuid) -> Result<Thread>;
    async fn add_message(&self, id: Uuid, content: &str) -> Result<()>;
}

pub struct InMemoryThreadsService {
    threads: ArcSwap<HashMap<Uuid, Thread>>,
}

impl InMemoryThreadsService {
    pub fn new() -> Self {
        let thread1 = Thread {
            id: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            messages: vec!["hello world".to_string(), "can you help?".to_string()],
        };
        let threads = ArcSwap::from_pointee(HashMap::from([(thread1.id, thread1)]));
        InMemoryThreadsService { threads }
    }
}

#[async_trait]
impl ThreadsService for InMemoryThreadsService {
    async fn create(&self) -> Result<Thread> {
        let thread = Thread {
            id: Uuid::new_v4(),
            messages: Vec::new(),
        };

        self.threads.rcu(|threads| {
            let mut fresh = HashMap::clone(&threads);
            fresh.insert(thread.id, thread.clone());
            fresh
        });

        Ok(thread)
    }

    async fn list(&self) -> Vec<Thread> {
        self.threads.load().values().cloned().collect()
    }

    async fn get(&self, id: Uuid) -> Result<Thread> {
        self.threads
            .load()
            .get(&id)
            .ok_or(anyhow!("Could not find Thread with id {}", id))
            .map(|x| x.clone())
    }

    async fn add_message(&self, id: Uuid, content: &str) -> Result<()> {
        let mut thread = self.get(id).await?;
        thread.messages.push(content.to_string());

        self.threads.rcu(|threads| {
            let mut fresh = HashMap::clone(&threads);
            fresh.insert(thread.id, thread.clone());
            fresh
        });

        Ok(())
    }
}
