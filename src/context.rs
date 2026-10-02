use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};
use bon::Builder;

use crate::config::shared::SharedConfig;

#[derive(Builder)]
#[builder(
    builder_type(name = ServerContextBuilder, vis = "pub"),
    state_mod(name = server_context_builder, vis = "pub"),
    finish_fn(name = build_inner, vis = "")
)]
pub struct ServerContextInner {
    pub config: Arc<SharedConfig>,
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct ServerContext(Arc<ServerContextInner>);

impl ServerContext {
    pub fn builder() -> ServerContextBuilder {
        ServerContextInner::builder()
    }
}

impl<S: server_context_builder::State> ServerContextBuilder<S> {
    pub fn build(self) -> ServerContext
    where
        S: server_context_builder::IsComplete,
    {
        ServerContext(Arc::new(self.build_inner()))
    }
}

impl Deref for ServerContext {
    type Target = ServerContextInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
