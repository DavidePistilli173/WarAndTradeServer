use crate::game::game_date::GameDate;
use crate::protocol::gui_to_server::NewGamePld;

/// Data for a single game.
pub struct GameData {
    /// Name of the player's civilisation.
    civ_name: String,
    /// Current game date.
    date: GameDate,
}

impl GameData {
    /// Create a new, empty game.
    pub fn new() -> Self {
        Self {
            civ_name: String::from("NO NAME"),
            date: GameDate::new(1, 1, 1),
        }
    }

    /// Create a new game from some settings.
    pub fn from_settings(settings: &NewGamePld) -> Self {
        Self {
            civ_name: String::from_utf8_lossy(&settings.civ_name).into_owned(),
            date: GameDate::new(1, 1, 1),
        }
    }
}
