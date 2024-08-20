use crate::config::Settings;
use crate::threads;
use arc_swap::ArcSwap;
use axum::Router;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
        })
    }
}

pub fn build_router(state: Arc<ApplicationState>) -> Router {
    Router::new().nest("/threads", threads::router::build_router(state.clone()))
}
