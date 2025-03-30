use crate::game::common::GameSpeed;
use crate::game::date::GameDate;
use crate::protocol::{cmd, tlm};
use crossbeam_channel::{Receiver, Sender};
use rwlog::sender::Logger;

/// Overall state of the server containing the latest telemetries from the server.
pub struct ServerState {
    pub game_status: tlm::GameStatusPld,
    pub saved_games: tlm::SavedGamesPld,
    pub civ_data: tlm::CivDataPld,
}

impl ServerState {
    /// Create a new server state.
    pub fn new() -> Self {
        Self {
            game_status: tlm::GameStatusPld {
                ongoing: false,
                speed: GameSpeed::Paused,
                date: GameDate::new(0, 0, 1),
            },
            saved_games: tlm::SavedGamesPld {
                saved_games: Vec::new(),
            },
            civ_data: tlm::CivDataPld {
                civ_name: "NO NAME".to_string(),
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
                tlm::Tlm::GameStatus(pld) => {
                    server_state.game_status = pld;
                }
                tlm::Tlm::SavedGames(pld) => {
                    server_state.saved_games = pld;
                }
                tlm::Tlm::CivData(pld) => {
                    server_state.civ_data = pld;
                }
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
