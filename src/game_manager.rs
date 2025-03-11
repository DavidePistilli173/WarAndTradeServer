use core::str;
use std::fs::{self, File};
use std::io::Write;
use std::time::SystemTime;

use crate::game::common::{self, GameSpeed};
use crate::game::data::GameData;
use crate::protocol::common::STR_LEN;
use crate::protocol::server_to_gui::{self, GameStatusPld, MAX_SAVED_GAMES, SavedGamesPld};
use crate::protocol::{
    gui_to_server::{self, GuiToServerMsg},
    server_to_gui::ServerToGuiMsg,
};
use glob::glob;
use rwlog::sender::Logger;

/// Path where the save files are stored, relative to the executable.
pub const SAVE_FILE_PATH: &'static str = "saved_games";

/// Macro for packing a message.
macro_rules! pack_msg {
    ($vec:expr, $pack_res:expr, $enum_decl:expr) => {{
        if let Some(msg) = $pack_res {
            $vec.push($enum_decl(msg));
        }
    }};
}

/// Check whether enough time has elapsed before sending a telemetry.
macro_rules! update_telemetry_timer {
    ($logger:expr, $last_send_time:expr, $interval:expr) => {
        match SystemTime::now().duration_since($last_send_time) {
            Ok(elapsed) => {
                if elapsed < $interval {
                    false
                } else {
                    $last_send_time = SystemTime::now();
                    true
                }
            }
            Err(err) => {
                rwlog::err!($logger, "Failed to compute elapsed time: {err}");
                false
            }
        }
    };
}

/// Collection of timestamps when the different telemetries were last sent.
/// The GameStatus telemetry is sent on each, so its timer is not required.
pub struct TelemtryTimers {
    /// GameStatus
    pub game_status: SystemTime,
    /// SavedGames
    pub saved_games: SystemTime,
}

impl TelemtryTimers {
    /// Create a new telemetry timers object with all timers set to now.
    pub fn new() -> Self {
        let now = SystemTime::now();
        Self {
            game_status: now,
            saved_games: now,
        }
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
    /// List of saved games.
    saved_games: Vec<String>,
}

impl GameManager {
    /// Get the current telemetry to send.
    pub fn get_telemetries(&mut self) -> Vec<ServerToGuiMsg> {
        let mut result = Vec::new();
        pack_msg!(result, self.pack_game_status(), ServerToGuiMsg::GameStatus);
        pack_msg!(result, self.pack_saved_games(), ServerToGuiMsg::SavedGames);
        result
    }

    /// Create a new game object.
    pub fn new(logger: Logger) -> Self {
        // Initialise the game paths.
        if let Err(err) = fs::create_dir_all(SAVE_FILE_PATH) {
            rwlog::err!(&logger, "Failed to create save games path: {err}.");
        }

        GameManager {
            logger,
            ongoing: false,
            speed: common::GameSpeed::Paused,
            game_data: GameData::new(),
            telemtry_timers: TelemtryTimers::new(),
            saved_games: Vec::new(),
        }
    }

    /// Process a list of commands.
    pub fn process_commands(&mut self, commands: Vec<GuiToServerMsg>) {
        for cmd in commands.iter() {
            match cmd {
                GuiToServerMsg::NewGame(cmd_data) => {
                    self.process_new_game(cmd_data);
                }
                GuiToServerMsg::SaveGame(cmd_data) => {
                    self.process_save_game(cmd_data);
                }
                GuiToServerMsg::LoadGame(cmd_data) => {}
                GuiToServerMsg::DeleteSavedGame(cmd_data) => {}
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

    /// Update the list of currently saved files.
    fn update_saved_files(&mut self) {
        let mut paths = Vec::new();

        let file_list = glob(&format!("./{SAVE_FILE_PATH}/*.json"));
        if let Err(err) = file_list {
            rwlog::err!(
                &self.logger,
                "Failed to get the list of saved files: {err}."
            );
            return;
        }
        for file in file_list.unwrap() {
            match file {
                Ok(path) => paths.push(path),
                Err(err) => {
                    rwlog::warn!(&self.logger, "Failed to read save file path: {err}.");
                }
            }
        }

        self.saved_games = paths
            .iter()
            .filter_map(|path_buf| path_buf.to_str())
            .take(server_to_gui::MAX_SAVED_GAMES)
            .map(|str| str.to_string())
            .collect();
    }

    /// Pack the Game Status message.
    fn pack_game_status(&mut self) -> Option<GameStatusPld> {
        if !update_telemetry_timer!(
            &self.logger,
            self.telemtry_timers.game_status,
            server_to_gui::GAME_STATUS_TIME
        ) {
            return None;
        }

        Some(GameStatusPld {
            ongoing: self.ongoing,
            speed: self.speed,
            date: *self.game_data.date(),
        })
    }

    /// Pack the Save Game
    fn pack_saved_games(&mut self) -> Option<SavedGamesPld> {
        if !update_telemetry_timer!(
            &self.logger,
            self.telemtry_timers.saved_games,
            server_to_gui::SAVED_GAMES_TIME
        ) {
            return None;
        }

        self.update_saved_files();
        let mut array: [[u8; STR_LEN]; MAX_SAVED_GAMES] = [[0; STR_LEN]; MAX_SAVED_GAMES];
        for (i, raw_path) in self.saved_games.iter().enumerate() {
            array[i].copy_from_slice(&raw_path[..STR_LEN].as_bytes());
        }

        Some(SavedGamesPld { saved_games: array })
    }

    /// Process the NewGame command.
    fn process_new_game(&mut self, cmd_data: &gui_to_server::NewGamePld) {
        self.game_data = GameData::from_settings(cmd_data);
        self.speed = GameSpeed::Paused;
    }

    /// Process the SaveGame command.
    fn process_save_game(&self, cmd_data: &gui_to_server::SaveGamePld) {
        let name = str::from_utf8(&cmd_data.name).unwrap_or_else(|err| {
            rwlog::err!(
                &self.logger,
                "Failed to parse save game name: {err}. (Received {:?})",
                cmd_data.name
            );
            "default_path"
        });

        let path = format!("{SAVE_FILE_PATH}/{name}.json");
        let mut out_file = match File::create(path.clone()) {
            Ok(x) => x,
            Err(err) => {
                rwlog::err!(&self.logger, "Failed to open save file {path}: {err}.");
                return;
            }
        };

        let saved_json = match serde_json::to_string(&self.game_data) {
            Ok(x) => x,
            Err(err) => {
                rwlog::err!(&self.logger, "Failed to convert game data to json: {err}.");
                return;
            }
        };

        if let Err(err) = write!(out_file, "{saved_json}") {
            rwlog::err!(&self.logger, "Failed to write data to file: {err}.");
        }
    }
}
