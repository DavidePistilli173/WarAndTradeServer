use crate::game::date::GameDate;
use crate::protocol::gui_to_server::NewGamePld;
use serde::{Deserialize, Serialize};

/// Data for a single game.
#[derive(Serialize, Deserialize)]
pub struct GameData {
    /// Name of the player's civilisation.
    civ_name: String,
    /// Current game date.
    date: GameDate,
}

impl GameData {
    /// Get the current game date.
    pub fn date(&self) -> &GameDate {
        &self.date
    }

    /// Create a new game from some settings.
    pub fn from_settings(settings: &NewGamePld) -> Self {
        Self {
            civ_name: String::from_utf8_lossy(&settings.civ_name).into_owned(),
            date: GameDate::new(1, 1, 1),
        }
    }

    /// Create a new, empty game.
    pub fn new() -> Self {
        Self {
            civ_name: String::from("NO NAME"),
            date: GameDate::new(1, 1, 1),
        }
    }

    /// Simulate the game for a certain number of days.
    pub fn simulate(&mut self, days: u16) {
        self.date.add_days(days);
    }
}
