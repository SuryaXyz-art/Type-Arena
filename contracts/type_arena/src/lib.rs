pub mod state;

use linera_sdk::linera_base_types::{ContractAbi, ServiceAbi, ChainId};
use async_graphql::Request;
use serde::{Deserialize, Serialize};

pub use state::{TypeArenaState, Room, Tournament, PlayerStats};

pub struct TypeArenaAbi;

impl ContractAbi for TypeArenaAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for TypeArenaAbi {
    type Query = Request;
    type QueryResponse = async_graphql::Response;
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    CreateRoom { room_id: String, text: String },
    JoinRoom { room_id: String, host_chain_id: ChainId },
    SubmitResult { room_id: String, wpm: u32, time_ms: u64, host_chain_id: ChainId },
    FinishRoom { room_id: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TypeArenaEvent {
    RoomCreated { room_id: String },
    PlayerJoined { room_id: String, player: String },
    ResultSubmitted { room_id: String, player: String, wpm: u32 },
    RoomFinished { room_id: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    JoinRoom { room_id: String, player: String },
    SubmitResult { room_id: String, player: String, wpm: u32, time_ms: u64 },
}
