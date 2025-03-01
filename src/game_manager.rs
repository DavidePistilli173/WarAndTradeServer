use crate::game::game_data::GameData;
use crate::protocol::common;
use crate::protocol::server_to_gui::GameStatusPld;
use crate::protocol::{gui_to_server::GuiToServerMsg, server_to_gui::ServerToGuiMsg};
use rwlog::sender::Logger;

/// Struct containing the actual game logic.
pub struct GameManager {
    /// Logger.
    logger: Logger,
    /// True if a game is running, false otherwise.
    ongoing: bool,
    /// Current game speed.
    speed: common::GameSpeed,
    /// Data for the current game.
    game_data: GameData,
}

impl GameManager {
    /// Create a new game object.
    pub fn new(logger: Logger) -> Self {
        GameManager {
            logger,
            ongoing: false,
            speed: common::GameSpeed::Paused,
            game_data: GameData::new(),
        }
    }

    /// Process a list of commands.
    pub fn process_commands(&mut self, commands: Vec<GuiToServerMsg>) {
        for cmd in commands.iter() {
            match cmd {
                GuiToServerMsg::NewGame(cmd_data) => {
                    self.game_data = GameData::from_settings(cmd_data)
                }
                GuiToServerMsg::LoadGame(cmd_data) => {}
                GuiToServerMsg::SetSpeed(cmd_data) => self.speed = cmd_data.speed,
            }
        }
    }

    /// Get the current telemetry to send.
    pub fn get_telemetries(&self) -> Vec<ServerToGuiMsg> {
        let mut result = Vec::new();

        // Always send the basic game status.
        let msg = GameStatusPld {
            ongoing: self.ongoing,
            speed: self.speed,
        };
        result.push(ServerToGuiMsg::GameStatus(msg));

        result
    }
}
