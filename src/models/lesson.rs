use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

/// Represents a single lesson in a schedule.
///
/// Contains details about a specific class session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Title or name of the lesson
    pub title: String,
    /// Classroom or cabinet where the lesson takes place
    pub cabinet: String,
    /// Name of the teacher
    pub teacher: String,
    /// Order number of the lesson in the daily schedule
    pub order: u32,
    /// Start time of the lesson
    #[serde(rename = "startTime", with = "crate::utils::time_serde")]
    pub start_time: NaiveTime,
    /// End time of the lesson
    #[serde(rename = "endTime", with = "crate::utils::time_serde")]
    pub end_time: NaiveTime,
}
