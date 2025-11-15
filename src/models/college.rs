use super::{Call, Campus};
use serde::{Deserialize, Serialize};

/// Represents an educational institution.
///
/// Contains information about a college including its campuses and call schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct College {
    /// Unique identifier for the college
    #[serde(rename = "collegeId")]
    pub college_id: u32,
    /// Name of the college
    pub name: String,
    /// Schedule of calls (lesson periods) for the college
    pub calls: Vec<Call>,
    /// List of campuses belonging to this college
    pub campuses: Vec<Campus>,
}
