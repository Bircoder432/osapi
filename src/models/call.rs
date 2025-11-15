use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

/// Represents a call (lesson period) in the college schedule.
///
/// Defines the time periods for lessons throughout the day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    /// Unique identifier for the call
    #[serde(rename = "callId")]
    pub call_id: u32,
    /// Day of the week (1-7, where 1 is Monday)
    pub weekday: u8,
    /// Time when the lesson period begins
    #[serde(with = "crate::utils::time_serde")]
    pub begins: NaiveTime,
    /// Time when the lesson period ends
    #[serde(with = "crate::utils::time_serde")]
    pub ends: NaiveTime,
    /// Order of the call in the daily schedule
    pub order: u32,
}
