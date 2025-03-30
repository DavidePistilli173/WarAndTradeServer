use crate::protocol::cmd;
use crate::protocol::interface::{Interface, ServerState};
use egui::Ui;

use super::date_display::date_display_update;
use super::save_panel::SavePanel;
use super::speed_control::speed_control_update;

pub struct TopBar {
    pub save_panel: SavePanel,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            save_panel: SavePanel::new(),
        }
    }

    pub fn update(&mut self, ui: &mut Ui, interface: &Interface, server_state: &ServerState) {
        ui.horizontal(|ui| {
            self.save_panel.update(ui, interface, server_state);
            date_display_update(ui, interface, server_state);
            speed_control_update(ui, interface, server_state);
        });
    }
}
