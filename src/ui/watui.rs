//! War And Trade UI.

use crate::{
    protocol::{
        self,
        interface::{Interface, ServerState},
    },
    ui::gameplay::main_scene::MainScene,
    ui::main_menu::main_menu::MainMenuState,
};
use crossbeam_channel::{Receiver, Sender};
use rwlog::sender::Logger;

/// Current state of the UI.
struct UiState {
    /// Game's main menu.
    main_menu: MainMenuState,
    /// Main gameplay scene.
    main_scene: MainScene,
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
                main_scene: MainScene::new(),
            },
        }
    }
}

impl eframe::App for WATUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        self.interface.receive_telemetries(&mut self.server_state); // Get telemetries from the server.

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.server_state.game_running {
                self.ui_state
                    .main_scene
                    .update(ui, &self.interface, &self.server_state);
            } else {
                self.ui_state
                    .main_menu
                    .update(ui, &self.interface, &self.server_state);
            }
        });

        ctx.request_repaint();
    }
}
