//! Messages from the server to the GUI.

use crate::protocol::common;

/// Labels for messages from the server to the GUI.
#[repr(u8)]
pub enum ServerToGuiLabel {
    /// Status of the current game.
    GameStatus = 1,
}

pub enum ServerToGuiMsg {
    /// Status of the current game.
    GameStatus(GameStatusPld),
}

/// Header for all messages.
#[repr(C, packed(1))]
pub struct Header {
    /// Message label.
    label: ServerToGuiLabel,
}

/// Actual packet structure.
#[repr(C, packed(1))]
pub struct Packet<T> {
    /// Packet header.
    header: Header,
    /// Packet payload.
    payload: T,
}

/// Response to a connection request by a GUI.
#[repr(C, packed(1))]
pub struct GameStatusPld {
    /// True if a game is running, false otherwise.
    pub ongoing: bool,
    /// Current game speed.
    pub speed: common::GameSpeed,
}
