use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerAction {
    Move { player_id: String, x: f64, y: f64 },
    Attack { player_id: String, target_id: String, damage: u32 },
    UseItem { player_id: String, item_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MatchEvent {
    Start { match_id: String, players: Vec<String> },
    Kill { match_id: String, killer_id: String, victim_id: String },
    Score { match_id: String, player_id: String, points: u32 },
    End { match_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub player_id: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerSession {
    Login { player_id: String },
    Logout { player_id: String },
}
