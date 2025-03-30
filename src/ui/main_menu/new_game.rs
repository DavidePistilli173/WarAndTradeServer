use crate::protocol::cmd;
use crate::protocol::interface::Interface;
use crate::ui::main_menu::main_menu::MainMenuScene;
use egui::Ui;

pub struct NewGameState {
    pub civ_name: String,
}

impl NewGameState {
    pub fn update(&mut self, ui: &mut Ui, interface: &Interface, scene: &mut MainMenuScene) {
        ui.vertical_centered(|ui| {
            ui.text_edit_singleline(&mut self.civ_name);

            if ui.button("Start").clicked() {
                interface.send_command_to_server(cmd::Cmd::NewGame(cmd::NewGamePld {
                    civ_name: self.civ_name.clone(),
                }));
            }

            if ui.button("Back").clicked() {
                *scene = MainMenuScene::MainMenu;
            }
        });
    }
}
