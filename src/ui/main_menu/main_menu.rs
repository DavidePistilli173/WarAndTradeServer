use crate::protocol::cmd;
use crate::protocol::interface::Interface;
use crate::ui::main_menu::{
    load_game::LoadGameState, new_game::NewGameState, options::OptionsState,
};
use egui::Ui;

/// Overall scenes of the UI.
pub enum MainMenuScene {
    /// Main menu page.
    MainMenu,
    /// New game page of the main menu.
    NewGame,
    /// Load game page of the main menu.
    LoadGame,
    /// Options page of the main menu.
    Options,
}

pub struct MainMenuState {
    /// Current main emnu scene.
    scene: MainMenuScene,
    /// New game page of the main menu.
    new_game: NewGameState,
    /// Load game page of the main menu.
    load_game: LoadGameState,
    /// Options page of the main menu.
    options: OptionsState,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            scene: MainMenuScene::MainMenu,
            new_game: NewGameState {
                civ_name: "My Civ".to_string(),
            },
            load_game: LoadGameState {
                file_list: Vec::new(),
            },
            options: OptionsState {},
        }
    }

    pub fn update(&mut self, ui: &mut Ui, interface: &Interface) {
        match self.scene {
            MainMenuScene::MainMenu => {
                ui.vertical_centered(|ui| ui.heading("War And Trade"));
                ui.vertical_centered(|ui| {
                    if ui.button("New Game").clicked() {
                        self.scene = MainMenuScene::NewGame;
                    }

                    if ui.button("Load Game").clicked() {
                        self.scene = MainMenuScene::LoadGame;
                    }

                    if ui.button("Options").clicked() {
                        self.scene = MainMenuScene::Options;
                    }
                });
            }
            MainMenuScene::NewGame => self.new_game.update(ui, interface, &mut self.scene),
            MainMenuScene::LoadGame => self.load_game.update(ui, interface, &mut self.scene),
            MainMenuScene::Options => self.options.update(ui, interface, &mut self.scene),
        }
    }
}
