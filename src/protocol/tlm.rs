//! Messages from the server to the GUI.

use serde::{Deserialize, Serialize};

use crate::game::common;
use crate::game::world::World;

#[derive(Serialize, Deserialize, Debug)]
pub enum Tlm {
    /// Communicate that a game has started.
    GameStarted,
    /// Communicate that the running game has stopped.
    GameStopped,
    /// State of the current game.
    GameState(World),
    /// List of saved games.
    SavedGames(SavedGamesPld),
    /// The game speed has changed.
    SpeedChanged(common::GameSpeed),
}

/// List of available saved games.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SavedGamesPld {
    /// List of names of all available saved games.
    pub saved_games: Vec<String>,
}
