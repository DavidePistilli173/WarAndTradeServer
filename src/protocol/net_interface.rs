use rwlog::sender::Logger;
use std::net::UdpSocket;

use crate::protocol::gui_to_server::{self, SetSpeedPld};
use crate::protocol::gui_to_server::{GuiToServerLabel, GuiToServerMsg, LoadGamePld, NewGamePld};
use crate::protocol::server_to_gui::{ServerToGuiLabel, ServerToGuiMsg};

/// IP address the server listens to for GUI connections.
pub const SERVER_ADDRESS: &'static str = "127.0.0.1:18024";
/// Multicast group used by the server to stream information.
pub const TELEMETRY_DESTINATION: &'static str = "224.1.1.1:18025";

/// Macro for decoding an incoming message.
macro_rules! decode_msg {
    ($buff:expr, $payload_type:ty, $enum_decl:expr) => {{
        let msg: gui_to_server::Packet<$payload_type> =
            unsafe { std::ptr::read_unaligned($buff.as_ptr() as *const _) };
        $enum_decl(msg.payload)
    }};
}

/// Network interface for the game server.
pub struct NetInterface {
    /// Logger.
    logger: Logger,
    /// Main server socket.
    socket: UdpSocket,
}

impl NetInterface {
    pub fn get_commands(&self) -> Vec<GuiToServerMsg> {
        let mut result = Vec::new();

        // Process the received datagrams.
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

            // Decode the header.
            let header: gui_to_server::Header =
                unsafe { std::ptr::read_unaligned(buff.as_ptr() as *const _) };

            // Decode the actual packet.
            match header.label {
                GuiToServerLabel::NewGame => {
                    result.push(decode_msg!(buff, NewGamePld, GuiToServerMsg::NewGame));
                }
                GuiToServerLabel::LoadGame => {
                    result.push(decode_msg!(buff, LoadGamePld, GuiToServerMsg::LoadGame));
                }
                GuiToServerLabel::SetSpeed => {
                    result.push(decode_msg!(buff, SetSpeedPld, GuiToServerMsg::SetSpeed));
                }
            }

            processed_packets += 1;
        }

        result
    }

    /// Create a new network interface.
    pub fn new(logger: Logger) -> Result<Self, ()> {
        // Create the socket.
        let socket = UdpSocket::bind(SERVER_ADDRESS).map_err(|err| {
            rwlog::err!(&logger, "Failed to bind socket: {err}.");
            ()
        })?;

        // Set the socket as non-blocking.
        socket.set_nonblocking(true).map_err(|err| {
            rwlog::err!(&logger, "Failed to set the socket as non blocking: {err}.");
            ()
        })?;

        Ok(NetInterface { logger, socket })
    }

    pub fn send_telemetry(&self, telemetry: Vec<ServerToGuiMsg>) {}
}
