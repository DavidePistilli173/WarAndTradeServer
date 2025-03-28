//! Messages from the server to the GUI.

use serde::{Deserialize, Serialize};

use crate::game::common;
use crate::game::date::GameDate;

#[derive(Serialize, Deserialize)]
pub enum Tlm {
    GameStatus(GameStatusPld),
    SavedGames(SavedGamesPld),
    CivData(CivDataPld),
}

/// Basic server and game status, sent continuously.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct GameStatusPld {
    /// True if a game is running, false otherwise.
    pub ongoing: bool,
    /// Current game speed.
    pub speed: common::GameSpeed,
    /// Current game date.
    pub date: GameDate,
}

/// List of available saved games.
#[derive(Clone, Serialize, Deserialize)]
pub struct SavedGamesPld {
    /// List of names of all available saved games.
    pub saved_games: Vec<String>,
}

/// Player civilization data.
#[derive(Clone, Serialize, Deserialize)]
pub struct CivDataPld {
    /// Name of the civilization.
    pub civ_name: String,
}
