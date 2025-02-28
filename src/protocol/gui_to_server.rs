//! Messages from the GUI to the server.

use crate::protocol::common;

/// Labels for messages from the GUI to the server.
#[repr(u8)]
pub enum GuiToServerLabel {
    /// Start a new game.
    NewGame = 1,
    /// Load an existing game.
    LoadGame = 2,
    /// Set the game speed.
    SetSpeed = 3,
}

/// Actual enumeration containing all available messages.
pub enum GuiToServerMsg {
    /// Start a new game.
    NewGame(NewGamePld),
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
    pub civ_name: [u8; 64],
}

/// Keep-alive message.
#[repr(C, packed(1))]
pub struct LoadGamePld {
    /// ID of the game to load.
    pub id: u32,
}

#[repr(C, packed(1))]
pub struct SetSpeedPld {
    /// New speed to set.
    pub speed: common::GameSpeed,
}
