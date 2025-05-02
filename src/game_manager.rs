use core::str;
use std::fs::{self, File, remove_file};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::game;
use crate::game::common::GameSpeed;
use crate::protocol::{cmd, interface::ServerState, tlm};
use crossbeam_channel::{Receiver, Sender};
use glob::glob;
use rwlog::sender::Logger;

/// Path where the save files are stored, relative to the executable.
pub const SAVE_FILE_PATH: &'static str = "saved_games";

/// Time for each main loop iteration.
const ITERATION_TIME: Duration = Duration::from_millis(25);

/// Reference time for the game speed.
const REFERENCE_TIME: Duration = Duration::from_secs(1);

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
    state: ServerState,
    /// Time that needs to pass before simulating one day.
    time_per_day: Duration,
    /// Last time when the simulation was updated.
    last_sim_time: Duration,
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
            state: ServerState::new(),
            time_per_day: Duration::from_millis(0),
            last_sim_time: Duration::from_millis(0),
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
                cmd::Cmd::ReqSavedGamesList => {
                    self.send_tlm(tlm::Tlm::SavedGames(self.state.saved_games.clone()))
                }
                cmd::Cmd::LoadGame(cmd_data) => self.process_load_game(&cmd_data),
                cmd::Cmd::DeleteSavedGame(cmd_data) => {
                    self.process_delete_game(&cmd_data);
                }
                cmd::Cmd::SetSpeed(cmd_data) => {
                    self.set_speed(cmd_data.speed);
                }
                cmd::Cmd::StopGame => {
                    self.state.game_running = false;
                    self.send_tlm(tlm::Tlm::GameStopped);
                }
                cmd::Cmd::CloseServer() => {
                    self.active = false;
                }
            }
        }
    }

    // Main game loop. This function does not return.
    pub fn run(&mut self) {
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

    /// Send a single telemetry.
    fn send_tlm(&self, tlm: tlm::Tlm) {
        // Only log non-periodic telemetries.
        match tlm {
            tlm::Tlm::GameState(_) => {}
            _ => {
                rwlog::info!(&self.logger, "Sending telemetry: {:?}", tlm);
            }
        }

        // Send the telemetry.
        if let Err(err) = self.tlm_tx.send(tlm) {
            rwlog::err!(&self.logger, "Failed to send telemetry: {err}.");
        }
    }

    /// Send periodic telemetries.
    fn send_periodic_telemetry(&mut self) {
        self.update_saved_files();

        self.send_tlm(tlm::Tlm::GameState(self.state.game_state.clone()));
    }

    fn set_speed(&mut self, speed: GameSpeed) {
        self.state.game_speed = speed;

        match speed {
            GameSpeed::Paused => self.time_per_day = Duration::from_millis(0),
            GameSpeed::Speed1X => self.time_per_day = REFERENCE_TIME / 1,
            GameSpeed::Speed2X => self.time_per_day = REFERENCE_TIME / 2,
            GameSpeed::Speed4X => self.time_per_day = REFERENCE_TIME / 4,
            GameSpeed::Speed10X => self.time_per_day = REFERENCE_TIME / 10,
            GameSpeed::Speed40X => self.time_per_day = REFERENCE_TIME / 40,
        }

        self.send_tlm(tlm::Tlm::SpeedChanged(self.state.game_speed));
    }

    /// Run the game logic at the appropriate speed, if a game is running.
    fn simulate(&mut self) {
        match self.state.game_speed {
            GameSpeed::Paused => {}
            _ => {
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards.");

                if current_time - self.last_sim_time < self.time_per_day {
                    return;
                }

                self.last_sim_time = current_time;
                self.state.game_state.simulate();
            }
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

        self.state.saved_games.saved_games = paths
            .iter()
            .filter_map(|path_buf| path_buf.to_str())
            .map(|str| str.to_string())
            .collect();
    }

    /// Process the NewGame command.
    fn process_new_game(&mut self, cmd_data: &cmd::NewGamePld) {
        self.state.game_state = game::world::World::from_settings(cmd_data);

        self.set_speed(GameSpeed::Paused);

        self.state.game_running = true;
        self.send_tlm(tlm::Tlm::GameStarted);
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

        let saved_json = match serde_json::to_string(&self.state.game_state) {
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

    /// Process the LoadGame command.
    fn process_load_game(&mut self, cmd_data: &cmd::LoadGamePld) {
        if let Some(_) = self
            .state
            .saved_games
            .saved_games
            .iter()
            .find(|item| **item == cmd_data.name)
        {
            // Compose the file path.
            let file_path = &cmd_data.name;

            // Read and deserialize the data.
            let mut file = match File::open(&cmd_data.name) {
                Ok(x) => x,
                Err(err) => {
                    rwlog::err!(
                        &self.logger,
                        "Failed to open saved file: {file_path}, {err}"
                    );
                    return;
                }
            };

            let mut contents = String::new();
            if let Err(err) = file.read_to_string(&mut contents) {
                rwlog::err!(
                    &self.logger,
                    "Failed to read saved file: {file_path}, {err}"
                );
                return;
            }

            match serde_json::from_str(&contents) {
                Ok(x) => {
                    self.state.game_state = x;

                    self.set_speed(GameSpeed::Paused);

                    self.state.game_running = true;
                    self.send_tlm(tlm::Tlm::GameStarted);
                }
                Err(err) => {
                    rwlog::err!(
                        &self.logger,
                        "Failed to deserialise the saved game data: {file_path}, {err}"
                    );
                }
            }
        }
    }

    /// Process the DeleteGame command.
    fn process_delete_game(&mut self, cmd_data: &cmd::DeleteSavedGamePld) {
        if let Err(err) = remove_file(&cmd_data.name) {
            rwlog::err!(
                &self.logger,
                "Failed to delete saved game: {}, {err}",
                cmd_data.name
            );
        }

        self.update_saved_files();
    }
}
