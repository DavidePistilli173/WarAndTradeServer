use crate::ui::main_menu::main_menu::MainMenuScene;
use egui::Ui;

pub struct OptionsState {}

impl OptionsState {
    pub fn update(&mut self, ui: &mut Ui, scene: &mut MainMenuScene) {
        ui.vertical_centered(|ui| {
            if ui.button("Back").clicked() {
                *scene = MainMenuScene::MainMenu;
            }
        });
    }
}
