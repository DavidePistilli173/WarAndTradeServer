//! Shared state between the HTTP server and the game logic.

use serde::de;

use crate::game;
use crate::game::data::GameData;

/// Overall state of the game.
#[derive(Clone)]
pub struct GameState {
    /// True if a game is running, false otherwise.
    pub ongoing: bool,
    /// Current game speed.
    pub speed: game::common::GameSpeed,
    /// Data for the current game.
    pub game_data: GameData,
    /// List of saved games.
    pub saved_games: Vec<String>,
}

impl GameState {
    /// Create a new shared state object.
    pub fn new() -> Self {
        GameState {
            ongoing: false,
            speed: game::common::GameSpeed::Paused,
            game_data: GameData::new(),
            saved_games: Vec::new(),
        }
    }
}
