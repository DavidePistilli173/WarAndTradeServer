//! Messages from the server to the GUI.

use crate::protocol::common;

/// Labels for messages from the server to the GUI.
#[repr(u8)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct Header {
    /// Message label.
    pub label: ServerToGuiLabel,
}

/// Actual packet structure.
#[repr(C, packed(1))]
pub struct Packet<T> {
    /// Packet header.
    pub header: Header,
    /// Packet payload.
    pub payload: T,
}

/// Response to a connection request by a GUI.
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct GameStatusPld {
    /// True if a game is running, false otherwise.
    pub ongoing: bool,
    /// Current game speed.
    pub speed: common::GameSpeed,
}
