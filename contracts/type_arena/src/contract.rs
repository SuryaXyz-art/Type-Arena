#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TypeArenaState;
use linera_sdk::{
    Contract, ContractRuntime,
    views::{RootView, View},
    linera_base_types::{WithContractAbi, StreamName},
};
use type_arena::{TypeArenaAbi, Operation, Message, TypeArenaEvent};
use serde::{Deserialize, Serialize};

linera_sdk::contract!(TypeArena);

pub struct TypeArena {
    state: TypeArenaState,
    runtime: ContractRuntime<Self>,
}

impl WithContractAbi for TypeArena {
    type Abi = TypeArenaAbi;
}

impl Contract for TypeArena {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = ();
    type EventValue = TypeArenaEvent;

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
                let signer = self.runtime.authenticated_signer().map(|s| s.to_string()).unwrap_or_default();
                let start_time = self.runtime.system_time().micros(); 
                self.state.create_room(
                    room_id.clone(),
                    signer,
                    text,
                    start_time,
                ).await.expect("Failed to create room");
                self.runtime.emit(
                    StreamName::from("events"),
                    &TypeArenaEvent::RoomCreated { room_id }
                );
            }
            Operation::JoinRoom { room_id, host_chain_id } => {
                let player = self.runtime.authenticated_signer().map(|s| s.to_string()).unwrap_or_default();
                if host_chain_id == self.runtime.chain_id() {
                    self.state.join_room(room_id.clone(), player.clone()).await.expect("Failed to join room locally");
                    self.runtime.emit(
                        StreamName::from("events"),
                        &TypeArenaEvent::PlayerJoined { room_id, player }
                    );
                } else {
                    let message = Message::JoinRoom { room_id, player };
                    self.runtime.send_message(host_chain_id, message);
                }
            }
            Operation::SubmitResult { room_id, wpm, time_ms, host_chain_id } => {
                let player = self.runtime.authenticated_signer().map(|s| s.to_string()).unwrap_or_default();
                if host_chain_id == self.runtime.chain_id() {
                    self.state.submit_result(room_id.clone(), player.clone(), wpm, time_ms).await.expect("Failed to submit result locally");
                    self.runtime.emit(
                        StreamName::from("events"),
                        &TypeArenaEvent::ResultSubmitted { room_id, player, wpm }
                    );
                } else {
                    let message = Message::SubmitResult { room_id, player, wpm, time_ms };
                    self.runtime.send_message(host_chain_id, message);
                }
            }
            Operation::FinishRoom { room_id } => {
                self.state.finish_room(room_id.clone()).await.expect("Failed to finish room");
                 self.runtime.emit(
                    StreamName::from("events"),
                    &TypeArenaEvent::RoomFinished { room_id }
                );
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::JoinRoom { room_id, player } => {
                // In a real app, verify 'player' against message sender authentication if needed.
                // For now, we trust the message content for simplicity or assume signed messages.
                 self.state.join_room(room_id.clone(), player.clone()).await.expect("Failed to process JoinRoom message");
                 self.runtime.emit(
                    StreamName::from("events"),
                    &TypeArenaEvent::PlayerJoined { room_id, player }
                );
            }
            Message::SubmitResult { room_id, player, wpm, time_ms } => {
                 self.state.submit_result(room_id.clone(), player.clone(), wpm, time_ms).await.expect("Failed to process SubmitResult message");
                 self.runtime.emit(
                    StreamName::from("events"),
                    &TypeArenaEvent::ResultSubmitted { room_id, player, wpm }
                );
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
