use crate::{
    game::common::GameSpeed,
    protocol::{
        cmd::{self, SetSpeedPld},
        interface::{Interface, ServerState},
    },
};
use egui::Ui;

pub fn speed_control_update(ui: &mut Ui, interface: &Interface, server_state: &ServerState) {
    ui.horizontal_top(|ui| {
        let pause_btn = ui.add(egui::Button::image(egui::Image::from_bytes(
            "bytes://speed_control/paused",
            include_bytes!("../../../res/speed/pause.png"),
        )));
        let speed_1x_btn = ui.add(egui::Button::new("1X"));
        let speed_2x_btn = ui.add(egui::Button::new("2X"));
        let speed_4x_btn = ui.add(egui::Button::new("4X"));
        let speed_10x_btn = ui.add(egui::Button::new("10X"));
        let speed_40x_btn = ui.add(egui::Button::new("40X"));

        if pause_btn.clicked() {
            interface.send_command_to_server(cmd::Cmd::SetSpeed(SetSpeedPld {
                speed: GameSpeed::Paused,
            }));
        } else if speed_1x_btn.clicked() {
            interface.send_command_to_server(cmd::Cmd::SetSpeed(SetSpeedPld {
                speed: GameSpeed::Speed1X,
            }));
        } else if speed_2x_btn.clicked() {
            interface.send_command_to_server(cmd::Cmd::SetSpeed(SetSpeedPld {
                speed: GameSpeed::Speed2X,
            }));
        } else if speed_4x_btn.clicked() {
            interface.send_command_to_server(cmd::Cmd::SetSpeed(SetSpeedPld {
                speed: GameSpeed::Speed4X,
            }));
        } else if speed_10x_btn.clicked() {
            interface.send_command_to_server(cmd::Cmd::SetSpeed(SetSpeedPld {
                speed: GameSpeed::Speed10X,
            }));
        } else if speed_40x_btn.clicked() {
            interface.send_command_to_server(cmd::Cmd::SetSpeed(SetSpeedPld {
                speed: GameSpeed::Speed40X,
            }));
        }

        match server_state.game_speed {
            GameSpeed::Paused => pause_btn.highlight(),
            GameSpeed::Speed1X => speed_1x_btn.highlight(),
            GameSpeed::Speed2X => speed_2x_btn.highlight(),
            GameSpeed::Speed4X => speed_4x_btn.highlight(),
            GameSpeed::Speed10X => speed_10x_btn.highlight(),
            GameSpeed::Speed40X => speed_40x_btn.highlight(),
        }
    });
}
