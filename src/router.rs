use crate::config::Settings;
use crate::threads;
use axum::Router;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: Settings,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: settings.clone(),
        })
    }
}

pub fn build_router(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/threads", threads::router::build_router(state.clone()))
}
