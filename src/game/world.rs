use crate::game::date::GameDate;
use crate::protocol::cmd;
use serde::{Deserialize, Serialize};

/// Data for a single game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct World {
    /// Name of the player's civilisation.
    civ_name: String,
    /// Current game date.
    date: GameDate,
}

impl World {
    /// Get the current civilisation name.
    pub fn civ_name(&self) -> &String {
        &self.civ_name
    }

    /// Get the current game date.
    pub fn date(&self) -> &GameDate {
        &self.date
    }

    /// Create a new game from some settings.
    pub fn from_settings(settings: &cmd::NewGamePld) -> Self {
        Self {
            civ_name: settings.civ_name.clone(),
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
