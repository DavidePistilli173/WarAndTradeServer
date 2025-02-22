//! Errors for the rwgfx library.

use std::{error::Error, fmt};

/// Possible errors during mesh creation.
#[derive(Debug, Copy, Clone)]
pub enum AppManagerInitErr {
    /// Error while creating the socket for communicating with the GUI.
    SocketCreation,
}

impl Error for AppManagerInitErr {}

impl fmt::Display for AppManagerInitErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::SocketCreation => {
                write!(f, "Failed to create the GUI socket.")
            }
        }
    }
}
