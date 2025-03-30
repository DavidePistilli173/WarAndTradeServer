use crate::protocol::cmd;
use crate::protocol::interface::{Interface, ServerState};
use crate::ui::main_menu::main_menu::MainMenuScene;
use egui::Ui;

pub struct LoadGameState {}

impl LoadGameState {
    pub fn update(
        &mut self,
        ui: &mut Ui,
        interface: &Interface,
        scene: &mut MainMenuScene,
        server_state: &ServerState,
    ) {
        ui.vertical_centered(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in server_state.saved_games.saved_games.iter() {
                    if ui.button(file).clicked() {
                        interface.send_command_to_server(cmd::Cmd::LoadGame(cmd::LoadGamePld {
                            name: file.clone(),
                        }));
                    }
                }
            });

            if ui.button("Back").clicked() {
                *scene = MainMenuScene::MainMenu;
            }
        });
    }
}
