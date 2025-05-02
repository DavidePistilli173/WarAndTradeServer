use crate::game::civ::Civ;
use crate::game::date::GameDate;
use crate::protocol::cmd;
use serde::{Deserialize, Serialize};

/// Data for a single game.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct World {
    /// Player civilisation.
    player_civ: Civ,
    /// Current game date.
    date: GameDate,
}

impl World {
    /// Get the player's civilisation.
    pub fn player_civ(&self) -> &Civ {
        &self.player_civ
    }

    /// Get the current game date.
    pub fn date(&self) -> &GameDate {
        &self.date
    }

    /// Create a new game from some settings.
    pub fn from_settings(settings: &cmd::NewGamePld) -> Self {
        Self {
            player_civ: Civ::new(settings.civ_name.clone()),
            date: GameDate::new(1, 1, 1),
        }
    }

    /// Create a new, empty game.
    pub fn new() -> Self {
        Self {
            player_civ: Civ::new(String::from("NO NAME")),
            date: GameDate::new(1, 1, 1),
        }
    }

    /// Simulate the game for a certain number of days.
    pub fn simulate(&mut self) {
        self.player_civ.simulate();
        self.date.add_days(1);
    }
}
