#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TypeArenaState;
use self::state::{Room, PlayerStats};
use async_graphql::{Request, Response, Schema, Object, EmptyMutation, EmptySubscription};
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

struct QueryRoot {
    state: Arc<TypeArenaState>,
}

#[Object]
impl QueryRoot {
    async fn rooms(&self, key: String) -> Result<Option<Room>, Error> {
        Ok(self.state.rooms.get(&key).await?)
    }

    async fn player_stats(&self, key: String) -> Result<Option<PlayerStats>, Error> {
        Ok(self.state.player_stats.get(&key).await?)
    }
}

#[async_trait]
impl Service for TypeArena {
    type Error = Error;
    type Storage = ServiceStateStorage<Self>;

    async fn query_application(
        &self,
        _context: &QueryContext,
        argument: Request,
    ) -> Result<Response, Self::Error> {
        let schema = Schema::build(
            QueryRoot { state: self.state.clone() },
            EmptyMutation,
            EmptySubscription,
        )
        .finish();
        
        let response = schema.execute(argument).await;
        Ok(response)
    }
}
