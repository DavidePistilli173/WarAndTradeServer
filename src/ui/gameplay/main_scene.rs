use super::left_panel::LeftPanel;
use super::top_bar::TopBar;
use crate::protocol::cmd;
use crate::protocol::interface::{Interface, ServerState};
use egui::Ui;

pub struct MainScene {
    pub top_bar: TopBar,
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            top_bar: TopBar::new(),
        }
    }

    pub fn update(&mut self, ui: &mut Ui, interface: &Interface, server_state: &ServerState) {
        ui.vertical_centered(|ui| {
            self.top_bar.update(ui, interface, server_state);

            ui.horizontal(|ui| {
                LeftPanel::update(ui, interface, server_state);
            });

            if ui.button("Back To Main Menu").clicked() {
                interface.send_command_to_server(cmd::Cmd::StopGame);
            }
        });
    }
}
