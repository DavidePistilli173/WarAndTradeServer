use crate::protocol::interface::{Interface, ServerState};
use eframe::glow::DRAW_BUFFER;
use egui::Ui;

pub struct LeftPanel {}

impl LeftPanel {
    pub fn update(ui: &mut Ui, interface: &Interface, server_state: &ServerState) {
        let (food_satisfied, drink_satisfied) =
            server_state.game_state.player_civ().satisfied_needs();

        ui.vertical(|ui| {
            ui.label(server_state.game_state.player_civ().civ_name());
            ui.label(format!(
                "Population: {}",
                server_state.game_state.player_civ().population()
            ));
            ui.label(format!("Food needs: {}", food_satisfied * 100.0));
            ui.label(format!("Drink needs: {}", drink_satisfied * 100.0));
        });
    }
}
