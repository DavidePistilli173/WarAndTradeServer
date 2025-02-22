use crate::errors::AppManagerInitErr;
use crate::protocol::gui_to_server::{self, GuiToServerMsg};
use rwlog::sender::Logger;
use std::char::MAX;
use std::net::UdpSocket;
use std::os::unix::process;
use std::thread::sleep;
use std::time::Duration;

/// IP address the server listens to for GUI connections.
const SERVER_ADDRESS: &'static str = "127.0.0.1:18024";
/// Multicast group used by the server to stream information.
const MULTICAST_GROUP: &'static str = "224.1.1.1";

/// Application states.
pub enum AppState {
    /// Waiting for the initial connection by the GUI.
    WaitInitialConnection,
    /// Waiting for the game to start.
    WaitGameStart,
    /// Game running.
    GameRunning,
}

/// Main server application manager.
pub struct AppManager {
    /// Logger to use during the application's execution.
    logger: Logger,
    /// Socket used for communicating with the GUI.
    socket: UdpSocket,
    /// Current server state.
    state: AppState,
}

impl AppManager {
    /// Create a new server manager.
    pub fn new(logger: Logger) -> Result<Self, AppManagerInitErr> {
        let socket = UdpSocket::bind("127.0.0.1:18024").map_err(|err| {
            rwlog::err!(&logger, "Failed to bind socket: {err}");
            AppManagerInitErr::SocketCreation
        })?;

        Ok(AppManager {
            logger,
            socket,
            state: AppState::WaitInitialConnection,
        })
    }

    /// Run the application.
    pub fn run(&mut self) {
        loop {
            let commands = self.get_commands();

            match self.state {
                AppState::WaitInitialConnection => self.process_initial_connection(),
                AppState::WaitGameStart => self.process_game_start(),
                AppState::GameRunning => self.process_game_running(),
            }
        }
    }

    /// Get commands from the socket.
    fn get_commands(&mut self) -> Vec<GuiToServerMsg> {
        let result = Vec::new();

        // Process all received datagrams.
        const MAX_PACKETS: u8 = 10; // Maximum number of packets to process.
        let mut processed_packets: u8 = 0;
        while MAX_PACKETS > processed_packets {
            let mut buff: [u8; size_of::<GuiToServerMsg>()] = [0; size_of::<GuiToServerMsg>()];
            let read_res = self.socket.recv_from(&mut buff);

            // No more data can be read.
            if read_res.is_err() {
                return result;
            }
            let read_bytes = read_res.unwrap().0;

            // Not enough bytes received to decode a header.
            if size_of::<gui_to_server::Header>() > read_bytes {
                continue;
            }

            unsafe {
                let header: gui_to_server::Header =
                    unsafe { std::ptr::read_unaligned(buff.as_ptr() as *const _) };
            }

            processed_packets += 1;
        }

        result
    }

    /// Process the wait and reception of the initial connection from the GUI.
    fn process_initial_connection(&mut self) {
        rwlog::info!(
            &self.logger,
            "Waiting connection from the GUI on {SERVER_ADDRESS}."
        );
        sleep(Duration::from_millis(1000));
    }

    /// Wait and process the game start command.
    fn process_game_start(&mut self) {}

    /// Process the game running state.
    fn process_game_running(&mut self) {}
}
