use core::str;
use std::fs::{self, File};
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::game;
use crate::game_state::GameState;
use crate::protocol::{cmd, tlm};
use crossbeam_channel::{Receiver, Sender};
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
    /// Channel for sending telemetries.
    tlm_tx: Sender<tlm::Tlm>,
    /// Loop control variable.
    active: bool,
    /// State variable used for running the game logic.
    state: GameState,
}

impl GameManager {
    /// Create a new game object.
    pub fn new(logger: Logger, cmd_rx: Receiver<cmd::Cmd>, tlm_tx: Sender<tlm::Tlm>) -> Self {
        // Initialise the game paths.
        if let Err(err) = fs::create_dir_all(SAVE_FILE_PATH) {
            rwlog::err!(&logger, "Failed to create save games path: {err}.");
        }

        GameManager {
            logger,
            cmd_rx,
            tlm_tx,
            active: false,
            state: GameState::new(),
        }
    }

    /// Process a list of commands.
    fn process_commands(&mut self) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
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
                cmd::Cmd::CloseServer() => {
                    self.active = false;
                }
            }
        }
    }

    // Main game loop. This function does not return.
    pub fn run(&mut self) {
        const ITERATION_TIME: Duration = Duration::from_millis(33);
        self.active = true;

        while self.active {
            let start = SystemTime::now();
            let start = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            self.process_commands();
            self.simulate();
            self.send_periodic_telemetry();

            let end = SystemTime::now();
            let end = end
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards.");

            sleep(ITERATION_TIME - (end - start));
        }
    }

    /// Send periodic telemetries.
    fn send_periodic_telemetry(&mut self) {
        self.update_saved_files();

        let tlm = tlm::Tlm::GameStatus(tlm::GameStatusPld {
            ongoing: self.state.ongoing,
            speed: self.state.speed,
            date: *self.state.game_data.date(),
        });

        if let Err(err) = self.tlm_tx.send(tlm) {
            rwlog::err!(&self.logger, "Failed to send telemetry: {err}.");
        }

        let tlm = tlm::Tlm::SavedGames(tlm::SavedGamesPld {
            saved_games: self.state.saved_games.clone(),
        });

        if let Err(err) = self.tlm_tx.send(tlm) {
            rwlog::err!(&self.logger, "Failed to send telemetry: {err}.");
        }

        if self.state.ongoing {
            let tlm = tlm::Tlm::CivData(tlm::CivDataPld {
                civ_name: self.state.game_data.civ_name().clone(),
            });
            if let Err(err) = self.tlm_tx.send(tlm) {
                rwlog::err!(&self.logger, "Failed to send telemetry: {err}.");
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
        self.state.ongoing = true;
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
