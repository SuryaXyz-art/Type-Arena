#![cfg_attr(target_arch = "wasm32", no_main)]

use crate::state::TypeArenaState;
use linera_sdk::{
    Contract, ContractRuntime,
    views::{RootView, View},
};
use crate::TypeArenaAbi;
use serde::{Deserialize, Serialize};

linera_sdk::contract!(TypeArena);

pub struct TypeArena {
    state: TypeArenaState,
    runtime: ContractRuntime<Self>,
}

impl linera_sdk::abi::WithContractAbi for TypeArena {
    type Abi = TypeArenaAbi;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    CreateRoom { room_id: String, text: String },
    SubmitResult { room_id: String, wpm: u32, time_ms: u64 },
    FinishRoom { room_id: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    // Cross-chain messages if needed
}

impl Contract for TypeArena {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TypeArenaState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TypeArena { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // Init
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::CreateRoom { room_id, text } => {
                let chain_id = self.runtime.chain_id().to_string();
                let signer = self.runtime.authenticated_signer().map(|s| s.to_string()).unwrap_or_default();
                self.state.create_room(
                    room_id,
                    signer,
                    text,
                    chain_id.parse().unwrap_or(0),
                ).await.expect("Failed to create room");
            }
            Operation::SubmitResult { room_id, wpm, time_ms } => {
                let signer = self.runtime.authenticated_signer().map(|s| s.to_string()).unwrap_or_default();
                self.state.submit_result(
                    room_id,
                    signer,
                    wpm,
                    time_ms,
                ).await.expect("Failed to submit result");
            }
            Operation::FinishRoom { room_id } => {
                self.state.finish_room(room_id).await.expect("Failed to finish room");
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {
        // Handle cross-chain messages
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linera_views::memory::create_memory_context;
    use crate::state::StateError;

    #[tokio::test]
    async fn test_create_and_join_room() {
        let context = create_memory_context();
        let mut state = TypeArenaState::load(context).await.unwrap();

        let host = "owner_key".to_string();

        // 1. Create Room
        let room_id = "ROOM_123".to_string();
        state.create_room(room_id.clone(), host.clone(), "Hello world".to_string(), 0).await.unwrap();

        // Verify room exists
        let room = state.rooms.get(&room_id).await.unwrap();
        assert!(room.is_some());
        assert_eq!(room.unwrap().host, host);

        // 2. Submit Result
        state.submit_result(room_id.clone(), host.clone(), 120, 60000).await.unwrap();

        // Verify player result
        let room = state.rooms.get(&room_id).await.unwrap().unwrap();
        assert_eq!(room.players.len(), 1);
        assert_eq!(room.players[0].wpm, 120);

        // 3. Finish Room
        state.finish_room(room_id.clone()).await.unwrap();
        
        // Verify finished
        let room = state.rooms.get(&room_id).await.unwrap().unwrap();
        assert!(room.is_finished);

        // 4. Try submit after finish (should fail)
        let res = state.submit_result(room_id.clone(), host.clone(), 150, 50000).await;
        assert!(matches!(res, Err(StateError::RoomFinished)));
    }

    #[tokio::test]
    async fn test_error_cases() {
        let context = create_memory_context();
        let mut state = TypeArenaState::load(context).await.unwrap();

        let room_id = "ROOM_ERR".to_string();

        // 1. Submit to non-existent room
        let res = state.submit_result(room_id.clone(), "player".into(), 100, 60000).await;
        assert!(matches!(res, Err(StateError::RoomNotFound)));

        // 2. Create Room
        state.create_room(room_id.clone(), "host".into(), "Test".into(), 0).await.unwrap();

        // 3. Create Duplicate Room
        let res_dup = state.create_room(room_id.clone(), "host".into(), "Test".into(), 0).await;
        assert!(matches!(res_dup, Err(StateError::RoomExists)));

        // 4. Finish non-existent room
        let res_finish = state.finish_room("NON_EXISTENT".to_string()).await;
        assert!(matches!(res_finish, Err(StateError::RoomNotFound)));
    }
}
