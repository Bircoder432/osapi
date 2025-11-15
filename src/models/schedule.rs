use super::Lesson;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "groupId")]
    pub group_id: u32,
    #[serde(with = "crate::utils::date_serde")]
    pub date: NaiveDate,
    pub lessons: Vec<Lesson>,
}
