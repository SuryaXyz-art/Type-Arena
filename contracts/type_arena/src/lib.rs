pub mod state;
pub mod contract;
pub mod service;

use linera_sdk::abi::{ContractAbi, ServiceAbi};
use async_graphql::Request;
use crate::contract::{Operation, Message};

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
