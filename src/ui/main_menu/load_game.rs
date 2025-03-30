use crate::protocol::cmd;
use crate::protocol::interface::Interface;
use crate::ui::main_menu::main_menu::MainMenuScene;
use egui::Ui;

pub struct LoadGameState {
    pub file_list: Vec<String>,
}

impl LoadGameState {
    pub fn update(&mut self, ui: &mut Ui, interface: &Interface, scene: &mut MainMenuScene) {
        ui.vertical_centered(|ui| {
            if ui.button("Back").clicked() {
                *scene = MainMenuScene::MainMenu;
            }
        });
    }
}
