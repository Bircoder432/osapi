pub mod admin;
pub mod colleges;
pub mod groups;
pub mod parser;
pub mod schedules;

pub use admin::AdminApi;
pub use colleges::CampusQuery;
pub use colleges::CampusesQuery;
pub use colleges::CollegeQuery;
pub use colleges::CollegesQuery;
pub use groups::GroupsQuery;
pub use parser::ParserApi;
pub use schedules::ScheduleQuery;
