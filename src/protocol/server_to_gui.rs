//! Messages from the server to the GUI.

/// Labels for messages from the server to the GUI.
pub enum GuiToServerMsg {
    /// Used for the initial connection response.
    InitialConnectionResponse(InitialConnectionResponse),
}

/// Header for all messages.
#[repr(C, packed(1))]
pub struct Header {
    /// Message label. (GuiToServerLabel)
    label: u8,
}

/// Response to a connection request by a GUI.
#[repr(C, packed(1))]
pub struct InitialConnectionResponse {
    /// Message header.
    header: Header,
    /// Result of the connection request.
    accepted: bool,
}
