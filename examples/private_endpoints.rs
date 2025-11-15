use osars::{
    CallRequest, Client, CreateParserRequest, LessonRequest, UpdateCallsRequest,
    UpdateGroupsRequest, UpdateLessonsRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen");

    // Authenticated client with token
    let auth_client = client.authenticated().with_token("your_token_here");

    // Work with admin Endpoints
    let admin_api = auth_client.admin();

    // Create parser
    let create_request = CreateParserRequest {
        college_name: "TKPST".to_string(),
        campus_names: vec!["Lunacharskogo".to_string(), "Samartseva".to_string()],
    };

    let response = admin_api.create_parser(create_request).await?;
    println!("Created parser with token: {}", response.token);

    // Remove parser
    admin_api.delete_parser(123).await?;
    println!("Parser deleted");

    // Work with parser
    let parser_api = auth_client.parser();

    // Update groups
    let groups_request = UpdateGroupsRequest {
        campus_id: 1,
        student_group_names: vec!["Group A".to_string(), "Group B".to_string()],
    };
    parser_api.update_groups(groups_request).await?;

    // Update call schedule
    let calls_request = UpdateCallsRequest {
        calls: vec![CallRequest {
            weekday: 1,
            begins: "09:00:00".to_string(),
            ends: "10:30:00".to_string(),
            order: 1,
        }],
    };
    parser_api.update_calls(calls_request).await?;

    // Add lessons
    let lessons_request = UpdateLessonsRequest {
        lessons: vec![LessonRequest {
            group_id: 1,
            order: 1,
            title: "Mathematics".to_string(),
            teacher: "Dr. Smith".to_string(),
            cabinet: "Room 101".to_string(),
            date: "2023-10-01".to_string(),
        }],
    };
    parser_api.add_lessons(lessons_request).await?;

    Ok(())
}
