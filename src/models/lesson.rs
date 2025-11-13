use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub title: String,
    pub cabinet: String,
    pub teacher: String,
    pub order: u32,
    #[serde(rename = "startTime", with = "crate::utils::time_serde")]
    pub start_time: NaiveTime,
    #[serde(rename = "endTime", with = "crate::utils::time_serde")]
    pub end_time: NaiveTime,
}
