use super::Group;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campus {
    #[serde(rename = "campusId")]
    pub campus_id: u32,
    pub name: String,
    #[serde(rename = "collegeId")]
    pub college_id: u32,
    #[serde(default)]
    pub groups: Vec<Group>,
}
