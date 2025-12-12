#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::{TypeArenaState, Room, PlayerResult, PlayerStats};
use async_trait::async_trait;
use linera_sdk::{
    base::{SessionId, WithContractAbi},
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

linera_sdk::contract!(TypeArena);

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("Room already exists")]
    RoomExists,
    #[error("Room not found")]
    RoomNotFound,
    #[error("Room already finished")]
    RoomFinished,
    #[error(transparent)]
    ViewError(#[from] linera_views::views::ViewError),
}

#[async_trait]
impl Contract for TypeArena {
    type Error = Error;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        _argument: (),
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::CreateRoom { room_id, text } => {
                if self.state.rooms.contains_key(&room_id).await? {
                    return Err(Error::RoomExists);
                }
                let room = Room {
                    id: room_id.clone(),
                    host: context.authenticated_signer.unwrap().to_string(),
                    text,
                    start_time: Some(context.chain_id.to_string().parse().unwrap_or(0)), // Mock timestamp
                    end_time: None,
                    players: vec![],
                    is_finished: false,
                };
                self.state.rooms.insert(&room_id, room)?;
            }
            Operation::SubmitResult { room_id, wpm, time_ms } => {
                let mut room = self.state.rooms.get(&room_id).await?.ok_or(Error::RoomNotFound)?;
                if room.is_finished {
                    return Err(Error::RoomFinished);
                }
                let player = context.authenticated_signer.unwrap().to_string();
                
                // Check if player already submitted?
                if room.players.iter().any(|p| p.address == player) {
                     // Update if better? Or reject?
                     // For now, allow update
                } else {
                     room.players.push(PlayerResult {
                         address: player.clone(),
                         wpm,
                         finish_time_ms: time_ms,
                     });
                }
                self.state.rooms.insert(&room_id, room)?;

                // Update Stats
                let mut stats = self.state.player_stats.get(&player).await?.unwrap_or_default();
                stats.total_races += 1;
                if wpm > stats.best_wpm {
                    stats.best_wpm = wpm;
                }
                self.state.player_stats.insert(&player, stats)?;
            }
            Operation::FinishRoom { room_id } => {
                 let mut room = self.state.rooms.get(&room_id).await?.ok_or(Error::RoomNotFound)?;
                 room.is_finished = true;
                 // Determine winner logic if needed here or just trust server submission?
                 // In trustless mode, we'd need better verification.
                 self.state.rooms.insert(&room_id, room)?;
            }
        }
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        _message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        _check: Self::ApplicationCall,
        _sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error> {
        Ok(ApplicationCallResult::default())
    }
    
    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        _session: Self::SessionState,
        _call: Self::SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error> {
        Ok(SessionCallResult::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linera_sdk::{
        base::{BlockHeight, ChainId, Owner, Timestamp, crypto},
        OperationContext,
    };
    use linera_views::memory::create_memory_context;
    // use linera_views::views::View; // No longer needed if we use TypeArenaState::load

    #[tokio::test]
    async fn test_create_and_join_room() {
        let context = create_memory_context();
        let mut contract = TypeArena::load(context).await.unwrap();

        let owner = Owner::from(crypto::PublicKey::test_key(1));
        let chain_id = ChainId::root(0);
        let op_context = OperationContext {
            chain_id,
            authenticated_signer: Some(owner),
            height: BlockHeight(0),
            index: 0,
            next_message_index: 0,
        };

        // 1. Create Room
        let room_id = "ROOM_123".to_string();
        let op = Operation::CreateRoom { room_id: room_id.clone(), text: "Hello world".to_string() };
        let result = contract.execute_operation(&op_context, op).await;
        assert!(result.is_ok());

        // Verify room exists (manual check or via state)
        let room = contract.state.rooms.get(&room_id).await.unwrap();
        assert!(room.is_some());
        assert_eq!(room.unwrap().host, owner.to_string());
        assert_eq!(contract.state.rooms.get(&room_id).await.unwrap().unwrap().text, "Hello world");

        // 2. Submit Result
        let op_submit = Operation::SubmitResult {
            room_id: room_id.clone(),
            wpm: 120,
            time_ms: 60000,
        };
        let result_submit = contract.execute_operation(&op_context, op_submit).await;
        assert!(result_submit.is_ok());

        // Verify player result
        let room = contract.state.rooms.get(&room_id).await.unwrap().unwrap();
        assert_eq!(room.players.len(), 1);
        assert_eq!(room.players[0].wpm, 120);

        // 3. Finish Room
        let op_finish = Operation::FinishRoom { room_id: room_id.clone() };
        let result_finish = contract.execute_operation(&op_context, op_finish).await;
        assert!(result_finish.is_ok());
        
        // Verify finished
        let room = contract.state.rooms.get(&room_id).await.unwrap().unwrap();
        assert!(room.is_finished);

        // 4. Try submit after finish (should fail)
         let op_fail = Operation::SubmitResult {
            room_id: room_id.clone(),
            wpm: 150,
            time_ms: 50000,
        };
        let result_fail = contract.execute_operation(&op_context, op_fail).await;
        assert!(matches!(result_fail, Err(Error::RoomFinished)));
    }

    #[tokio::test]
    async fn test_error_cases() {
        let context = create_memory_context();
        let mut contract = TypeArena::load(context).await.unwrap();

        let owner = Owner::from(crypto::PublicKey::test_key(1));
        let chain_id = ChainId::root(0);
        let op_context = OperationContext {
            chain_id,
            authenticated_signer: Some(owner),
            height: BlockHeight(0),
            index: 0,
            next_message_index: 0,
        };

        let room_id = "ROOM_ERR".to_string();

        // 1. Submit to non-existent room
        let op_submit_fail = Operation::SubmitResult {
            room_id: room_id.clone(),
            wpm: 100,
            time_ms: 60000,
        };
        let result_submit_fail = contract.execute_operation(&op_context, op_submit_fail).await;
        assert!(matches!(result_submit_fail, Err(Error::RoomNotFound)));

        // 2. Create Room
        let op_create = Operation::CreateRoom { room_id: room_id.clone(), text: "Test".to_string() };
        let result_create = contract.execute_operation(&op_context, op_create).await;
        assert!(result_create.is_ok());

        // 3. Create Duplicate Room
        let op_create_dup = Operation::CreateRoom { room_id: room_id.clone(), text: "Test".to_string() };
        let result_create_dup = contract.execute_operation(&op_context, op_create_dup).await;
        assert!(matches!(result_create_dup, Err(Error::RoomExists)));

        // 4. Finish non-existent room
        let op_finish_fail = Operation::FinishRoom { room_id: "NON_EXISTENT".to_string() };
        let result_finish_fail = contract.execute_operation(&op_context, op_finish_fail).await;
        assert!(matches!(result_finish_fail, Err(Error::RoomNotFound)));
    }
}
