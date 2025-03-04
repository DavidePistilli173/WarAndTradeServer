use std::path::PathBuf;
use std::time::SystemTime;

use crate::game::common::{self, GameSpeed};
use crate::game::data::GameData;
use crate::protocol::server_to_gui::GameStatusPld;
use crate::protocol::{gui_to_server::GuiToServerMsg, server_to_gui::ServerToGuiMsg};
use glob::glob;
use rwlog::sender::Logger;

/// Path where the save files are stored, relative to the executable.
pub const SAVE_FILE_PATH: &'static str = "saved_games";

/// Collection of timestamps when the different telemetries were last sent.
/// The GameStatus telemetry is sent on each, so its timer is not required.
pub struct TelemtryTimers {
    /// SavedGames
    pub saved_games: SystemTime,
}

impl TelemtryTimers {
    /// Create a new telemetry timers object with all timers set to now.
    pub fn new() -> Self {
        let now = SystemTime::now();
        Self { saved_games: now }
    }
}

/// Struct containing the actual game logic.
pub struct GameManager {
    /// Logger.
    logger: Logger,
    /// True if a game is running, false otherwise.
    ongoing: bool,
    /// Current game speed.
    speed: common::GameSpeed,
    /// Data for the current game.
    game_data: GameData,
    /// Timestamps whene each telemetry type was last sent.
    telemtry_timers: TelemtryTimers,
}

impl GameManager {
    /// Get the list of currently saved files.
    fn get_saved_files(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();

        let file_list = glob(&format!("./{SAVE_FILE_PATH}/*.json"));
        if let Err(err) = file_list {
            rwlog::err!(
                &self.logger,
                "Failed to get the list of saved files: {err}."
            );
            return result;
        }
        for file in file_list.unwrap() {
            match file {
                Ok(path) => result.push(path),
                Err(err) => {
                    rwlog::warn!(&self.logger, "Failed to read save file path: {err}.");
                }
            }
        }

        result
    }

    /// Get the current telemetry to send.
    pub fn get_telemetries(&self) -> Vec<ServerToGuiMsg> {
        let mut result = Vec::new();

        // Always send the basic game status.
        let msg = GameStatusPld {
            ongoing: self.ongoing,
            speed: self.speed,
            date: *self.game_data.date(),
        };
        result.push(ServerToGuiMsg::GameStatus(msg));

        result
    }

    /// Create a new game object.
    pub fn new(logger: Logger) -> Self {
        GameManager {
            logger,
            ongoing: false,
            speed: common::GameSpeed::Paused,
            game_data: GameData::new(),
            telemtry_timers: TelemtryTimers::new(),
        }
    }

    /// Process a list of commands.
    pub fn process_commands(&mut self, commands: Vec<GuiToServerMsg>) {
        for cmd in commands.iter() {
            match cmd {
                GuiToServerMsg::NewGame(cmd_data) => {
                    self.game_data = GameData::from_settings(cmd_data)
                }
                GuiToServerMsg::SaveGame(cmd_data) => {}
                GuiToServerMsg::LoadGame(cmd_data) => {}
                GuiToServerMsg::SetSpeed(cmd_data) => self.speed = cmd_data.speed,
            }
        }
    }

    /// Run the game logic at the appropriate speed, if a game is running.
    pub fn simulate(&mut self) {
        match self.speed {
            GameSpeed::Speed1X => self.game_data.simulate(1),
            GameSpeed::Speed2X => self.game_data.simulate(2),
            GameSpeed::Speed4X => self.game_data.simulate(4),
            GameSpeed::Speed10X => self.game_data.simulate(10),
            GameSpeed::Speed40X => self.game_data.simulate(40),
            _ => {}
        }
    }
}
