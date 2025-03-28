//! War And Trade UI.

use crate::{
    game,
    protocol::{self, cmd, tlm},
};
use crossbeam_channel::{Receiver, Sender, unbounded};
use egui::Ui;
use rwlog::sender::Logger;

/// Overall scenes of the UI.
enum UiScene {
    /// Main menu page.
    MainMenu,
    /// New game page of the main menu.
    NewGame,
    /// Load game page of the main menu.
    LoadGame,
    /// Options page of the main menu.
    Options,
    /// Actual gameplay screen.
    Gameplay,
}

/// Overall state of the server containing the latest telemetries from the server.
struct ServerState {
    game_status: tlm::GameStatusPld,
    saved_games: tlm::SavedGamesPld,
    civ_data: tlm::CivDataPld,
}

struct NewGameState {
    civ_name: String,
}

/// Current state of the UI.
struct UiState {
    new_game: NewGameState,
}

pub struct WATUI {
    /// Logger.
    logger: Logger,
    /// Channel for sending commands to the server.
    cmd_tx: Sender<protocol::cmd::Cmd>,
    /// Channel for receiving telemetry from the server.
    tlm_rx: Receiver<protocol::tlm::Tlm>,
    /// Active UI scene.
    scene: UiScene,
    /// Current server state.
    server_state: ServerState,
    /// Current UI state.
    ui_state: UiState,
}

impl WATUI {
    /// Create a new UI instance.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        logger: Logger,
        cmd_tx: Sender<protocol::cmd::Cmd>,
        tlm_rx: Receiver<protocol::tlm::Tlm>,
    ) -> Self {
        Self {
            logger,
            cmd_tx,
            tlm_rx,
            scene: UiScene::MainMenu,
            server_state: ServerState {
                game_status: tlm::GameStatusPld {
                    ongoing: false,
                    speed: game::common::GameSpeed::Paused,
                    date: game::date::GameDate::new(0, 0, 1),
                },
                saved_games: tlm::SavedGamesPld {
                    saved_games: Vec::new(),
                },
                civ_data: tlm::CivDataPld {
                    civ_name: "NO NAME".to_string(),
                },
            },
            ui_state: UiState {
                new_game: NewGameState {
                    civ_name: "My Civ".to_string(),
                },
            },
        }
    }

    /// Draw and handle events from the main menu.
    fn handle_main_menu(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            if ui.button("New Game").clicked() {
                self.scene = UiScene::NewGame;
            }

            if ui.button("Load Game").clicked() {
                self.scene = UiScene::LoadGame;
            }

            if ui.button("Options").clicked() {
                self.scene = UiScene::Options;
            }
        });
    }

    fn handle_new_game(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.text_edit_singleline(&mut self.ui_state.new_game.civ_name);

            if ui.button("Start").clicked() {
                self.send_command_to_server(cmd::Cmd::NewGame(cmd::NewGamePld {
                    civ_name: self.ui_state.new_game.civ_name.clone(),
                }));
            }

            if ui.button("Back").clicked() {
                self.scene = UiScene::MainMenu;
            }
        });
    }

    fn handle_load_game(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            if ui.button("Back").clicked() {
                self.scene = UiScene::MainMenu;
            }
        });
    }

    fn handle_options(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            if ui.button("Back").clicked() {
                self.scene = UiScene::MainMenu;
            }
        });
    }

    fn handle_gameplay(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(&self.server_state.civ_data.civ_name);

            if ui.button("Back").clicked() {
                self.scene = UiScene::MainMenu;
            }
        });
    }

    /// Process server telemetries.
    fn process_tlm(&mut self) {
        while let Ok(tlm) = self.tlm_rx.try_recv() {
            match tlm {
                tlm::Tlm::GameStatus(pld) => {
                    self.process_game_status(pld);
                }
                tlm::Tlm::SavedGames(pld) => {
                    self.server_state.saved_games = pld;
                }
                tlm::Tlm::CivData(pld) => {}
            }
        }
    }

    /// Process a new game status telemetry.
    fn process_game_status(&mut self, pld: tlm::GameStatusPld) {
        if pld.ongoing {
            self.scene = UiScene::Gameplay;
        }

        self.server_state.game_status = pld;
    }

    fn send_command_to_server(&mut self, cmd: cmd::Cmd) {
        if let Err(err) = self.cmd_tx.send(cmd) {
            rwlog::err!(&self.logger, "Failed to send command to the server: {err}.");
        }
    }
}

impl eframe::App for WATUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.process_tlm(); // Get telemetries from the server.

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("War And Trade"));
            match self.scene {
                UiScene::MainMenu => {
                    self.handle_main_menu(ui);
                }
                UiScene::NewGame => {
                    self.handle_new_game(ui);
                }
                UiScene::LoadGame => {
                    self.handle_load_game(ui);
                }
                UiScene::Options => {
                    self.handle_options(ui);
                }
                UiScene::Gameplay => {
                    self.handle_gameplay(ui);
                }
            }
        });
    }
}
