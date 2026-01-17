#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::{TypeArenaState, Room, PlayerStats};
use async_graphql::{Schema, Object, EmptySubscription};
use linera_sdk::{
    Service, ServiceRuntime, 
    views::View, 
    linera_base_types::{WithServiceAbi, ChainId}
};
use std::sync::Arc;
use type_arena::{TypeArenaAbi, Operation};

linera_sdk::service!(TypeArena);

pub struct TypeArena {
    state: Arc<TypeArenaState>,
    runtime: ServiceRuntime<Self>,
}

impl WithServiceAbi for TypeArena {
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

    async fn room(&self, room_id: String) -> Option<Room> {
        self.state.rooms.get(&room_id).await.ok().flatten()
    }

    async fn player_stats(&self, key: String) -> Option<PlayerStats> {
        self.state.player_stats.get(&key).await.ok().flatten()
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_room(&self, room_id: String, text: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::CreateRoom { room_id, text }).unwrap()
    }

    async fn join_room(&self, room_id: String, host_chain_id: ChainId) -> Vec<u8> {
        bcs::to_bytes(&Operation::JoinRoom { room_id, host_chain_id }).unwrap()
    }

    async fn submit_result(&self, room_id: String, wpm: u32, time_ms: u64, host_chain_id: ChainId) -> Vec<u8> {
        bcs::to_bytes(&Operation::SubmitResult { room_id, wpm, time_ms, host_chain_id }).unwrap()
    }

    async fn finish_room(&self, room_id: String) -> Vec<u8> {
        bcs::to_bytes(&Operation::FinishRoom { room_id }).unwrap()
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
            MutationRoot,
            EmptySubscription,
        )
        .finish();

        schema.execute(query).await
    }
}
