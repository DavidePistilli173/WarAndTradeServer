//! Common type definitions.

use serde::{Deserialize, Serialize};

/// Available game speed levels.
#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub enum GameSpeed {
    /// The game is paused.
    Paused,
    /// The game is proceeding at its normal speed.
    Speed1X,
    /// The game is proceeding at 2x the normal speed.
    Speed2X,
    /// The game is proceeding at 4x the normal speed.
    Speed4X,
    /// The game is proceeding at 10x the normal speed.
    Speed10X,
    /// The game is proceeding at 40x the normal speed.
    Speed40X,
}
