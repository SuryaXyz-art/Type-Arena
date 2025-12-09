#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TypeArenaState;
use async_trait::async_trait;
use linera_sdk::{
    base::WithServiceAbi, Service, ServiceStateStorage, QueryContext,
};
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(TypeArena);

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),
    #[error(transparent)]
    GraphQL(#[from] async_graphql::Error),
}

#[async_trait]
impl Service for TypeArena {
    type Error = Error;
    type Storage = ServiceStateStorage<Self>;

    async fn query_application(
        &self,
        _context: &QueryContext,
        argument: async_graphql::Request,
    ) -> Result<async_graphql::Response, Self::Error> {
        let schema = async_graphql::Schema::build(
            self.state.clone(),
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .finish();
        
        let response = schema.execute(argument).await;
        Ok(response)
    }
}
