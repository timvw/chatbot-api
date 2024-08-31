use crate::router::ApplicationState;
use crate::threads::model::Thread;
use anyhow::{anyhow, Result};
use arc_swap::ArcSwap;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use axum::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::{uuid, Uuid};

#[async_trait]
pub trait ThreadsService {
    async fn create(&self) -> Result<Thread>;
    async fn list(&self) -> Vec<Thread>;
    async fn get(&self, id: Uuid) -> Result<Thread>;
    async fn add_message(&self, id: Uuid, content: &str) -> Result<String>;
}

#[derive(Debug)]
pub struct InMemoryThreadsService {
    application_state: Arc<ApplicationState>,
    threads: ArcSwap<HashMap<Uuid, Thread>>,
}

impl InMemoryThreadsService {
    pub fn new(application_state: Arc<ApplicationState>) -> Self {
        let thread1 = Thread {
            id: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
            messages: vec!["hello world".to_string(), "can you help?".to_string()],
        };
        let threads = ArcSwap::from_pointee(HashMap::from([(thread1.id, thread1)]));
        InMemoryThreadsService {
            application_state,
            threads,
        }
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
            let mut fresh = HashMap::clone(threads);
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
            .cloned()
    }


    #[tracing::instrument]
    async fn add_message(&self, id: Uuid, content: &str) -> Result<String> {
        let mut thread = self.get(id).await?;

        let client = Client::new();


        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(self.application_state.settings.config.max_tokens)
            .model(self.application_state.settings.config.model.clone())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a helpful assistant.")
                    .build()?
                    .into(),
                /*
                ChatCompletionRequestUserMessageArgs::default()
                    .content("Who won the world series in 2020?")
                    .build()?
                    .into(),
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content("The Los Angeles Dodgers won the World Series in 2020.")
                    .build()?
                    .into(),
                 */
                ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            ])
            .build()?;

        let response = client.chat().create(request).await?;

        println!("\nResponse:\n");
        for choice in response.choices.clone() {
            println!(
                "{}: Role: {}  Content: {:?}",
                choice.index, choice.message.role, choice.message.content
            );
        }

        let response = response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .clone()
            .unwrap();

        thread.messages.push(content.to_string());
        thread.messages.push(response.clone());

        self.threads.rcu(|threads| {
            let mut fresh = HashMap::clone(threads);
            fresh.insert(thread.id, thread.clone());
            fresh
        });

        Ok(response)
    }
}
