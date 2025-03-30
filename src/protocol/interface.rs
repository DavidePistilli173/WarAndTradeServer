use crate::game::common::GameSpeed;
use crate::game::world::World;
use crate::protocol::{cmd, tlm};
use crossbeam_channel::{Receiver, Sender};
use rwlog::sender::Logger;

/// Overall state of the server containing the latest telemetries from the server.
pub struct ServerState {
    pub game_running: bool,
    pub game_speed: GameSpeed,
    pub game_state: World,
    pub saved_games: tlm::SavedGamesPld,
}

impl ServerState {
    /// Create a new server state.
    pub fn new() -> Self {
        Self {
            game_running: false,
            game_speed: GameSpeed::Paused,
            game_state: World::new(),
            saved_games: tlm::SavedGamesPld {
                saved_games: Vec::new(),
            },
        }
    }
}

/// Interface between the server and the frontend.
pub struct Interface {
    /// Logger.
    logger: Logger,
    /// Channel for sending commands to the server.
    cmd_tx: Sender<cmd::Cmd>,
    /// Channel for receiving telemetry from the server.
    tlm_rx: Receiver<tlm::Tlm>,
}

impl Interface {
    /// Create a new interface with the server.
    pub fn new(logger: Logger, cmd_tx: Sender<cmd::Cmd>, tlm_rx: Receiver<tlm::Tlm>) -> Self {
        Self {
            logger,
            cmd_tx,
            tlm_rx,
        }
    }

    /// Receive telemetries from the server and store them.
    pub fn receive_telemetries(&self, server_state: &mut ServerState) {
        while let Ok(tlm) = self.tlm_rx.try_recv() {
            match tlm {
                tlm::Tlm::GameState(_) => {}
                _ => {
                    rwlog::info!(&self.logger, "Received telemetry: {:?}", tlm);
                }
            }

            match tlm {
                tlm::Tlm::GameStarted => server_state.game_running = true,
                tlm::Tlm::GameStopped => server_state.game_running = false,
                tlm::Tlm::GameState(new_state) => server_state.game_state = new_state,
                tlm::Tlm::SavedGames(list) => server_state.saved_games = list,
                tlm::Tlm::SpeedChanged(new_speed) => server_state.game_speed = new_speed,
            }
        }
    }

    /// Send a command to the server.
    pub fn send_command_to_server(&self, cmd: cmd::Cmd) {
        if let Err(err) = self.cmd_tx.send(cmd) {
            rwlog::err!(&self.logger, "Failed to send command to the server: {err}.");
        }
    }
}
