//! Commands from the GUI.

use crate::game;

/// Commands from the GUI.
pub enum Cmd {
    /// Start a new game.
    NewGame(NewGamePld),
    /// Save the current game.
    SaveGame(SaveGamePld),
    /// Load an existing game.
    LoadGame(LoadGamePld),
    /// Delete an existing saved game.
    DeleteSavedGame(DeleteSavedGamePld),
    /// Set the game speed.
    SetSpeed(SetSpeedPld),
}

/// Command for starting a new game.
pub struct NewGamePld {
    /// Name of the player's civilisation.
    pub civ_name: String,
}

pub struct SaveGamePld {
    /// Name of the save file (with no extension).
    /// If a save with the same name already exists, it will get overwritten.
    pub name: String,
}

pub struct LoadGamePld {
    /// Name of the game to load.
    /// If a game is currently running, it will be terminated without saving.
    pub name: String,
}

pub struct DeleteSavedGamePld {
    /// Name of the game to delete.
    pub name: String,
}

pub struct SetSpeedPld {
    /// New speed to set.
    pub speed: game::common::GameSpeed,
}
