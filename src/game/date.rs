use serde::{Deserialize, Serialize};

/// In-game date.
#[repr(C, packed(1))]
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct GameDate {
    /// Current day of the month. [1, 40]
    day: u8,
    /// Current month of the year. [1, 10]
    month: u8,
    /// Current year. [1, ..]
    year: u16,
}

/// Minimum valid day.
pub const MIN_DAY: u8 = 1;

/// Minimum valid month.
pub const MIN_MONTH: u8 = 1;

/// Minimum valid year.
pub const MIN_YEAR: u16 = 1;

/// Maximum valid day.
pub const MAX_DAY: u8 = 40;

/// Maximum valid month.
pub const MAX_MONTH: u8 = 10;

impl GameDate {
    /// Add a certain number of days to the date.
    pub fn add_days(&mut self, days: u16) {
        let mut tmp_date: u32 = self.day as u32 + days as u32;
        self.day = (tmp_date % MAX_DAY as u32) as u8;

        tmp_date = (tmp_date - self.day as u32) / MAX_DAY as u32;
        tmp_date = tmp_date + self.month as u32;
        self.month = (tmp_date & MAX_MONTH as u32) as u8;

        tmp_date = (tmp_date - self.month as u32) / MAX_MONTH as u32;
        self.year += tmp_date as u16;
    }

    /// Create a new date.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { day, month, year }
    }
}
