use linera_sdk::views::{linera_views, MapView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum StateError {
    RoomExists,
    RoomNotFound,
    RoomFinished,
    ViewError(linera_sdk::views::ViewError),
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StateError::RoomExists => write!(f, "Room already exists"),
            StateError::RoomNotFound => write!(f, "Room not found"),
            StateError::RoomFinished => write!(f, "Room already finished"),
            StateError::ViewError(e) => write!(f, "View error: {}", e),
        }
    }
}

impl std::error::Error for StateError {}

impl From<linera_sdk::views::ViewError> for StateError {
    fn from(error: linera_sdk::views::ViewError) -> Self {
        StateError::ViewError(error)
    }
}

#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct TypeArenaState {
    pub rooms: MapView<String, Room>,
    pub tournaments: MapView<String, Tournament>,
    pub player_stats: MapView<String, PlayerStats>,
}

impl TypeArenaState {
    pub async fn create_room(
        &mut self,
        room_id: String,
        host: String,
        text: String,
        start_time: u64,
    ) -> Result<(), StateError> {
        if self.rooms.contains_key(&room_id).await? {
            return Err(StateError::RoomExists);
        }
        let room = Room {
            id: room_id.clone(),
            host,
            text,
            start_time: Some(start_time),
            end_time: None,
            players: vec![],
            participants: vec![],
            is_finished: false,
        };
        self.rooms.insert(&room_id, room)?;
        Ok(())
    }

    pub async fn join_room(&mut self, room_id: String, player: String) -> Result<(), StateError> {
        let mut room = self.rooms.get(&room_id).await?.ok_or(StateError::RoomNotFound)?;
        if room.is_finished {
             return Err(StateError::RoomFinished);
        }
        if !room.participants.contains(&player) {
            room.participants.push(player);
            self.rooms.insert(&room_id, room)?;
        }
        Ok(())
    }

    pub async fn submit_result(
        &mut self,
        room_id: String,
        player: String,
        wpm: u32,
        time_ms: u64,
    ) -> Result<(), StateError> {
        let mut room = self.rooms.get(&room_id).await?.ok_or(StateError::RoomNotFound)?;
        if room.is_finished {
             return Err(StateError::RoomFinished);
        }
        
        if !room.players.iter().any(|p| p.address == player) {
             room.players.push(PlayerResult {
                 address: player.clone(),
                 wpm,
                 finish_time_ms: time_ms,
             });
             self.rooms.insert(&room_id, room)?;

             let mut stats = self.player_stats.get(&player).await?.unwrap_or_default();
             stats.total_races += 1;
             if wpm > stats.best_wpm {
                 stats.best_wpm = wpm;
             }
             self.player_stats.insert(&player, stats)?;
        }
        Ok(())
    }

    pub async fn finish_room(&mut self, room_id: String) -> Result<(), StateError> {
        let mut room = self.rooms.get(&room_id).await?.ok_or(StateError::RoomNotFound)?;
        room.is_finished = true;
        self.rooms.insert(&room_id, room)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default, async_graphql::SimpleObject)]
pub struct Room {
    pub id: String,
    pub host: String,
    pub text: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub players: Vec<PlayerResult>,
    pub participants: Vec<String>,
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