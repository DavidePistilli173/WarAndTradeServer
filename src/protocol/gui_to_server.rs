//! Messages from the GUI to the server.

use crate::game::common;

/// Length of strings passed through the network.
pub const STR_LEN: usize = 32;

/// Labels for messages from the GUI to the server.
#[repr(u8)]
pub enum GuiToServerLabel {
    /// Start a new game.
    NewGame = 1,
    /// Save the current game.
    SaveGame = 2,
    /// Load an existing game.
    LoadGame = 3,
    /// Set the game speed.
    SetSpeed = 4,
}

/// Actual enumeration containing all available messages.
pub enum GuiToServerMsg {
    /// Start a new game.
    NewGame(NewGamePld),
    /// Save the current game.
    SaveGame(SaveGamePld),
    /// Load an existing game.
    LoadGame(LoadGamePld),
    /// Set the game speed.
    SetSpeed(SetSpeedPld),
}

/// Header for all messages.
#[repr(C, packed(1))]
pub struct Header {
    /// Message label.
    pub label: GuiToServerLabel,
}

/// Actual packet structure.
#[repr(C, packed(1))]
pub struct Packet<T> {
    /// Packet header.
    pub header: Header,
    //// Packet payload.
    pub payload: T,
}

/// Command for starting a new game.
#[repr(C, packed(1))]
pub struct NewGamePld {
    /// Name of the player's civilisation.
    pub civ_name: [u8; STR_LEN],
}

#[repr(C, packed(1))]
pub struct SaveGamePld {
    /// Name of the save file (with no extension).
    /// If a save with the same name already exists, it will get overwritten.
    pub name: [u8; STR_LEN],
}

/// Keep-alive message.
#[repr(C, packed(1))]
pub struct LoadGamePld {
    /// Name of the game to load.
    /// If a game is currently running, it will be terminated without saving.
    pub name: [u8; STR_LEN],
}

#[repr(C, packed(1))]
pub struct SetSpeedPld {
    /// New speed to set.
    pub speed: common::GameSpeed,
}
