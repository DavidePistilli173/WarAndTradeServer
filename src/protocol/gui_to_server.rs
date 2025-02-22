//! Messages from the GUI to the server.

/// Labels for messages from the GUI to the server.
pub enum GuiToServerMsg {
    /// Used to start a new game.
    NewGame(NewGame),
    /// Used to load a previously saved game.
    LoadGame(LoadGame),
    /// Used to save the current game.
    SaveGame(SaveGame),
}

/// Header for all messages.
#[repr(C, packed(1))]
pub struct Header {
    /// ID of the sender, assigned from the server.
    /// On the initial connection request, this shall be ignored.
    /// On subsequent messages it will be used to identify the sender.
    sender_id: u64,
    /// Message label. (GuiToServerLabel)
    label: u8,
}

/// Command for starting a new game.
#[repr(C, packed(1))]
pub struct NewGame {
    /// Packet header.
    header: Header,
}

/// Command for loading a previously saved game.
#[repr(C, packed(1))]
pub struct LoadGame {
    /// Packet header.
    header: Header,
}

/// Command for saving the current game.
#[repr(C, packed(1))]
pub struct SaveGame {
    /// Packet header.
    header: Header,
}
