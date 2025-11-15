use super::Group;
use serde::{Deserialize, Serialize};

/// Represents a campus (branch) of a college.
///
/// A campus is a physical location where educational activities take place.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campus {
    /// Unique identifier for the campus
    #[serde(rename = "campusId")]
    pub id: u32,
    /// Name of the campus
    pub name: String,
    /// ID of the parent college
    #[serde(rename = "collegeId")]
    pub college_id: u32,
    /// List of student groups at this campus
    #[serde(default)]
    pub groups: Vec<Group>,
}
