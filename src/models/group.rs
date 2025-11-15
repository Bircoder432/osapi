use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "studentGroupId")]
    pub group_id: u32,
    pub name: String,
    #[serde(rename = "campusId")]
    pub campus_id: u32,
}
