// tests/lesson.rs
use osapi::models::Lesson;

#[test]
fn parse_camel_case_time() {
    let json = r#"{
        "title": "Math",
        "cabinet": "101",
        "teacher": "John",
        "order": 1,
        "startTime": "09:00:00",
        "endTime": "10:30:00"
    }"#;

    let lesson: Lesson = serde_json::from_str(json).unwrap();
    assert_eq!(lesson.start_time.to_string(), "09:00:00");
    assert_eq!(lesson.end_time.to_string(), "10:30:00");
}
