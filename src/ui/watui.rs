//! War And Trade UI.

use crate::{
    protocol::{
        self,
        interface::{Interface, ServerState},
    },
    ui::main_menu::main_menu::MainMenuState,
};
use crossbeam_channel::{Receiver, Sender};
use egui::Ui;
use rwlog::sender::Logger;

/// Current state of the UI.
struct UiState {
    /// Game's main menu.
    main_menu: MainMenuState,
}

pub struct WATUI {
    /// Logger.
    logger: Logger,
    /// Server interface.
    interface: Interface,
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
            logger: logger.clone(),
            interface: Interface::new(logger, cmd_tx, tlm_rx),
            server_state: ServerState::new(),
            ui_state: UiState {
                main_menu: MainMenuState::new(),
            },
        }
    }

    fn handle_gameplay(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label(&self.server_state.civ_data.civ_name);
        });
    }
}

impl eframe::App for WATUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.interface.receive_telemetries(&mut self.server_state); // Get telemetries from the server.

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.server_state.game_status.ongoing {
                self.handle_gameplay(ui);
            } else {
                self.ui_state.main_menu.update(ui, &self.interface);
            }
        });
    }
}
