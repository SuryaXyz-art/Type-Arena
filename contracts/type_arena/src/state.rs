use linera_sdk::views::{MapView, RegisterView, ViewStorageContext};
use linera_views::views::{GraphQLView, RootView};
use serde::{Deserialize, Serialize};

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct TypeArenaState {
    pub rooms: MapView<String, Room>,
    pub tournaments: MapView<String, Tournament>,
    pub player_stats: MapView<String, PlayerStats>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default, async_graphql::SimpleObject)]
pub struct Room {
    pub id: String,
    pub host: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub players: Vec<PlayerResult>,
    pub is_finished: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default, async_graphql::SimpleObject)]
pub struct PlayerResult {
    pub address: String,
    pub wpm: u32,
    pub finish_time_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default, async_graphql::SimpleObject)]
pub struct Tournament {
    pub id: String,
    pub host: String,
    pub max_players: u32,
    pub current_round: u32,
    pub participants: Vec<String>,
    pub winner: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default, async_graphql::SimpleObject)]
pub struct PlayerStats {
    pub wins: u32,
    pub total_races: u32,
    pub best_wpm: u32,
}
