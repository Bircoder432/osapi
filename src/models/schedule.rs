use super::Lesson;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "groupId")]
    pub group_id: u32,
    pub date: DateTime<Utc>,
    pub lessons: Vec<Lesson>,
}
