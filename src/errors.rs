//! Errors for the rwgfx library.

use std::{error::Error, fmt};

/// Possible errors during mesh creation.
#[derive(Debug, Copy, Clone)]
pub enum AppManagerInitErr {
    /// Error while creating the network interface for communicating with the GUIs.
    NetworkInterfaceCreation,
}

impl Error for AppManagerInitErr {}

impl fmt::Display for AppManagerInitErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NetworkInterfaceCreation => {
                write!(f, "Failed to create the server's network interface.")
            }
        }
    }
}
