use serde::{Deserialize, Serialize};

/// Represents a student group.
///
/// A group of students who attend classes together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Unique identifier for the group
    #[serde(rename = "studentGroupId")]
    pub id: u32,
    /// Name of the group
    pub name: String,
    /// ID of the campus where this group is located
    #[serde(rename = "campusId")]
    pub campus_id: u32,
}
