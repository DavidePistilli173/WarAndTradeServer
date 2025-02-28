//! Common type definitions.

/// Available game speed levels.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GameSpeed {
    /// The game is paused.
    Paused = 0,
    /// The game is proceeding at its normal speed.
    Speed1X = 1,
    /// The game is proceeding at 2x the normal speed.
    Speed2X = 2,
    /// The game is proceeding at 4x the normal speed.
    Speed4X = 3,
    /// The game is proceeding at 10x the normal speed.
    Speed10X = 4,
    /// The game is proceeding at 40x the normal speed.
    Speed40X = 5,
}
