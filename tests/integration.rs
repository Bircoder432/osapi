use osars::{Campus, Client, College, Group, Schedule};

#[tokio::test]
async fn test_client_workflow() {
    // Note: This would require a mock server in a real test environment
    let client = Client::new("https://api.example.com");

    // Test that we can create all query types without panicking
    let colleges_query = client.colleges();
    let _groups_query = client.groups(1);
    let _schedule_query = client.schedule(1);
    let _today_query = client.today(1);
    let _tomorrow_query = client.tomorrow(1);

    // This is just to verify the types compile correctly
    assert!(true);
}

#[test]
fn test_model_defaults() {
    // Test that models can be created with reasonable defaults
    let college = College {
        college_id: 1,
        name: "Test".to_string(),
        calls: vec![],
        campuses: vec![],
    };

    let campus = Campus {
        id: 1,
        name: "Test Campus".to_string(),
        college_id: 1,
        groups: vec![],
    };

    let group = Group {
        id: 1,
        name: "Group A".to_string(),
        campus_id: 1,
    };

    assert_eq!(college.college_id, 1);
    assert_eq!(campus.id, 1);
    assert_eq!(group.id, 1);
}
