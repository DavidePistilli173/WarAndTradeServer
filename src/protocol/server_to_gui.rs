//! Messages from the server to the GUI.

use crate::game::common;
use crate::game::date::GameDate;
use std::time::Duration;

use crate::protocol::common::STR_LEN;

/// Maximum number of saved games.
pub const MAX_SAVED_GAMES: usize = 32;

/// Interval between game status messages.
pub const GAME_STATUS_TIME: Duration = Duration::from_millis(10);
/// Interval between saved games messages.
pub const SAVED_GAMES_TIME: Duration = Duration::from_secs(2);

/// Labels for messages from the server to the GUI.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ServerToGuiLabel {
    /// Status of the current game.
    GameStatus = 1,
    /// List of available saved games.
    SavedGames = 2,
}

pub enum ServerToGuiMsg {
    /// Status of the current game.
    GameStatus(GameStatusPld),
    /// List of available saved games.
    SavedGames(SavedGamesPld),
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

/// Basic server and game status, sent continuously.
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct GameStatusPld {
    /// True if a game is running, false otherwise.
    pub ongoing: bool,
    /// Current game speed.
    pub speed: common::GameSpeed,
    /// Current game date.
    pub date: GameDate,
}

/// List of available saved games.
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SavedGamesPld {
    /// List of names of all available saved games.
    pub saved_games: [[u8; STR_LEN]; MAX_SAVED_GAMES],
}
