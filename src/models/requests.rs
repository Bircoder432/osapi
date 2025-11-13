pub struct UpdateGroupsRequest {
    pub campus_id: u32,
    pub student_group_names: Vec<String>,
}

pub struct CreateParserRequest {
    pub college_name: String,
    pub campus_names: Vec<String>,
}
