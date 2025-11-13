use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub call_id: u32,
    pub weekday: u8,
    #[serde(with = "crate::utils::time_serde")]
    pub begins: NaiveTime,
    #[serde(with = "crate::utils::time_serde")]
    pub ends: NaiveTime,
    pub order: u32,
}
