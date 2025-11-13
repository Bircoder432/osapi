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
pub use schedule::Schedule;

pub enum Week {
    Previous,
    Current,
    Next,
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub enum Day {
    Today,
    Tomorrow,
}
