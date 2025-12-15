#![cfg_attr(target_arch = "wasm32", no_main)]

use crate::state::TypeArenaState;
use crate::state::{Room, PlayerStats};
use async_graphql::{Request, Response, Schema, Object, EmptyMutation, EmptySubscription};
use linera_sdk::{Service, ServiceRuntime, views::View};
use std::sync::Arc;
use crate::TypeArenaAbi;

linera_sdk::service!(TypeArena);

pub struct TypeArena {
    state: Arc<TypeArenaState>,
    runtime: ServiceRuntime<Self>,
}

impl linera_sdk::abi::WithServiceAbi for TypeArena {
    type Abi = TypeArenaAbi;
}

struct QueryRoot {
    state: Arc<TypeArenaState>,
}

#[Object]
impl QueryRoot {
    async fn rooms(&self, key: String) -> Option<Room> {
        self.state.rooms.get(&key).await.ok().flatten()
    }

    async fn player_stats(&self, key: String) -> Option<PlayerStats> {
        self.state.player_stats.get(&key).await.ok().flatten()
    }
}

impl Service for TypeArena {
    type Parameters = ();
    
    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TypeArenaState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TypeArena { state: Arc::new(state), runtime }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot { state: self.state.clone() },
            EmptyMutation,
            EmptySubscription,
        )
        .finish();
        
        schema.execute(query).await
    }
}
