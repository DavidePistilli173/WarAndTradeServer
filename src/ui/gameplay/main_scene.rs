use crate::protocol::cmd;
use crate::protocol::interface::{Interface, ServerState};
use crate::ui::gameplay::speed_control::speed_control_update;
use egui::Ui;

pub struct MainScene {
    pub save_game_name: String,
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            save_game_name: "my_save_game".to_string(),
        }
    }

    pub fn update(&mut self, ui: &mut Ui, interface: &Interface, server_state: &ServerState) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.save_game_name);
                if ui.button("Save").clicked() {
                    interface.send_command_to_server(cmd::Cmd::SaveGame(cmd::SaveGamePld {
                        name: self.save_game_name.clone(),
                    }));
                }

                speed_control_update(ui, interface, server_state);
            });

            ui.label(server_state.game_state.civ_name());

            if ui.button("Back To Main Menu").clicked() {
                interface.send_command_to_server(cmd::Cmd::StopGame);
            }
        });
    }
}
