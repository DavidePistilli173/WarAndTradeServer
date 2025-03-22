use core::str;
use std::fs::{self, File};
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::game;
use crate::game_state::GameState;
use crate::protocol::{cmd, tlm};
use crossbeam_channel::Receiver;
use glob::glob;
use rwlog::sender::Logger;
use std::sync::{Arc, Mutex};

/// Path where the save files are stored, relative to the executable.
pub const SAVE_FILE_PATH: &'static str = "saved_games";

/// Struct containing the actual game logic.
pub struct GameManager {
    /// Logger.
    logger: Logger,
    /// User commands.
    cmd_rx: Receiver<cmd::Cmd>,
    /// State variable used for running the game logic.
    state: GameState,
    /// Game state shared with the external interface.
    shared_state: Arc<Mutex<GameState>>,
}

impl GameManager {
    /// Create a new game object.
    pub fn new(
        logger: Logger,
        shared_state: Arc<Mutex<GameState>>,
        cmd_rx: Receiver<cmd::Cmd>,
    ) -> Self {
        // Initialise the game paths.
        if let Err(err) = fs::create_dir_all(SAVE_FILE_PATH) {
            rwlog::err!(&logger, "Failed to create save games path: {err}.");
        }

        GameManager {
            logger,
            cmd_rx,
            state: GameState::new(),
            shared_state,
        }
    }

    /// Process a list of commands.
    fn process_commands(&mut self) {
        let commands: Vec<cmd::Cmd> = self.cmd_rx.iter().collect();
        for cmd in commands {
            match cmd {
                cmd::Cmd::NewGame(cmd_data) => {
                    self.process_new_game(&cmd_data);
                }
                cmd::Cmd::SaveGame(cmd_data) => {
                    self.process_save_game(&cmd_data);
                }
                cmd::Cmd::LoadGame(cmd_data) => {}
                cmd::Cmd::DeleteSavedGame(cmd_data) => {}
                cmd::Cmd::SetSpeed(cmd_data) => self.state.speed = cmd_data.speed,
            }
        }
    }

    // Main game loop. This function does not return.
    pub fn run(&mut self) {
        const ITERATION_TIME: Duration = Duration::from_millis(10);

        loop {
            let start = SystemTime::now();
            let start = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            self.process_commands();
            self.simulate();
            self.save_state();

            let end = SystemTime::now();
            let end = end
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards.");

            sleep(ITERATION_TIME - (end - start));
        }
    }

    /// Save the current game state to the shared state.
    fn save_state(&mut self) {
        match self.shared_state.lock() {
            Ok(mut shared_state) => *shared_state = self.state.clone(),
            Err(err) => {
                rwlog::err!(&self.logger, "Failed to lock shared state mutex: {err}.");
            }
        }
    }

    /// Run the game logic at the appropriate speed, if a game is running.
    fn simulate(&mut self) {
        match self.state.speed {
            game::common::GameSpeed::Speed1X => self.state.game_data.simulate(1),
            game::common::GameSpeed::Speed2X => self.state.game_data.simulate(2),
            game::common::GameSpeed::Speed4X => self.state.game_data.simulate(4),
            game::common::GameSpeed::Speed10X => self.state.game_data.simulate(10),
            game::common::GameSpeed::Speed40X => self.state.game_data.simulate(40),
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

        self.state.saved_games = paths
            .iter()
            .filter_map(|path_buf| path_buf.to_str())
            .map(|str| str.to_string())
            .collect();
    }

    /// Process the NewGame command.
    fn process_new_game(&mut self, cmd_data: &cmd::NewGamePld) {
        self.state.game_data = game::data::GameData::from_settings(cmd_data);
        self.state.speed = game::common::GameSpeed::Paused;
    }

    /// Process the SaveGame command.
    fn process_save_game(&self, cmd_data: &cmd::SaveGamePld) {
        let path = format!("{SAVE_FILE_PATH}/{}.json", cmd_data.name);
        let mut out_file = match File::create(path.clone()) {
            Ok(x) => x,
            Err(err) => {
                rwlog::err!(&self.logger, "Failed to open save file {path}: {err}.");
                return;
            }
        };

        let saved_json = match serde_json::to_string(&self.state.game_data) {
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
