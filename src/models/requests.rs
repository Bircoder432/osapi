use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupsRequest {
    pub campus_id: u32,
    pub student_group_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParserRequest {
    pub college_name: String,
    pub campus_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCallsRequest {
    pub calls: Vec<CallRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRequest {
    pub weekday: u8,
    pub begins: String,
    pub ends: String,
    pub order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLessonsRequest {
    pub lessons: Vec<LessonRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonRequest {
    pub group_id: u32,
    pub order: u32,
    pub title: String,
    pub teacher: String,
    pub cabinet: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParserResponse {
    pub token: String,
}
