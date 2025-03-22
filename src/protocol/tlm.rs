//! Messages from the server to the GUI.

use crate::game::common;
use crate::game::date::GameDate;

/// Basic server and game status, sent continuously.
#[derive(Clone, Copy)]
pub struct GameStatusTlm {
    /// True if a game is running, false otherwise.
    pub ongoing: bool,
    /// Current game speed.
    pub speed: common::GameSpeed,
    /// Current game date.
    pub date: GameDate,
}

/// List of available saved games.
#[derive(Clone)]
pub struct SavedGamesTlm {
    /// List of names of all available saved games.
    pub saved_games: Vec<String>,
}
