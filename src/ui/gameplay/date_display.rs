use crate::protocol::interface::ServerState;
use egui::Ui;

pub fn date_display_update(ui: &mut Ui, server_state: &ServerState) {
    ui.horizontal_top(|ui| {
        ui.horizontal(|ui| {
            ui.label(format!(
                "{:0>4}/{:0>2}/{:0>2}",
                server_state.game_state.date().year,
                server_state.game_state.date().month,
                server_state.game_state.date().day,
            ));
        });
    });
}
