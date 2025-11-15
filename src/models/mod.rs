pub mod call;
pub mod campus;
pub mod college;
pub mod group;
pub mod lesson;
pub mod requests;
pub mod schedule;

pub use call::Call;
pub use campus::Campus;
pub use college::College;
pub use group::Group;
pub use lesson::Lesson;
pub use requests::*;
pub use schedule::Schedule;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Week {
    Previous,
    Current,
    Next,
}

#[derive(Debug, Clone)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone)]
pub enum Day {
    Today,
    Tomorrow,
}

impl fmt::Display for Week {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Week::Previous => write!(f, "previous"),
            Week::Current => write!(f, "current"),
            Week::Next => write!(f, "next"),
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weekday::Monday => write!(f, "monday"),
            Weekday::Tuesday => write!(f, "tuesday"),
            Weekday::Wednesday => write!(f, "wednesday"),
            Weekday::Thursday => write!(f, "thursday"),
            Weekday::Friday => write!(f, "friday"),
            Weekday::Saturday => write!(f, "saturday"),
            Weekday::Sunday => write!(f, "sunday"),
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day::Today => write!(f, "today"),
            Day::Tomorrow => write!(f, "tomorrow"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_week_display() {
        assert_eq!(Week::Previous.to_string(), "previous");
        assert_eq!(Week::Current.to_string(), "current");
        assert_eq!(Week::Next.to_string(), "next");
    }

    #[test]
    fn test_weekday_display() {
        assert_eq!(Weekday::Monday.to_string(), "monday");
        assert_eq!(Weekday::Tuesday.to_string(), "tuesday");
        assert_eq!(Weekday::Wednesday.to_string(), "wednesday");
        assert_eq!(Weekday::Thursday.to_string(), "thursday");
        assert_eq!(Weekday::Friday.to_string(), "friday");
        assert_eq!(Weekday::Saturday.to_string(), "saturday");
        assert_eq!(Weekday::Sunday.to_string(), "sunday");
    }

    #[test]
    fn test_day_display() {
        assert_eq!(Day::Today.to_string(), "today");
        assert_eq!(Day::Tomorrow.to_string(), "tomorrow");
    }

    #[test]
    fn test_college_serialization() {
        let college = College {
            college_id: 1,
            name: "Test College".to_string(),
            calls: vec![],
            campuses: vec![],
        };

        let json = serde_json::to_string(&college).unwrap();
        assert!(json.contains("\"collegeId\":1"));
        assert!(json.contains("\"name\":\"Test College\""));
    }

    #[test]
    fn test_lesson_serialization() {
        let lesson = Lesson {
            title: "Mathematics".to_string(),
            cabinet: "Room 101".to_string(),
            teacher: "Dr. Smith".to_string(),
            order: 1,
            start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        };

        let json = serde_json::to_string(&lesson).unwrap();
        assert!(json.contains("\"title\":\"Mathematics\""));
        assert!(json.contains("\"cabinet\":\"Room 101\""));
        assert!(json.contains("\"teacher\":\"Dr. Smith\""));
        assert!(json.contains("\"order\":1"));
    }
}
