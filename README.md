# OSARS Client Library

A comprehensive Rust client library for interacting with the OpenScheduleAPI. This library provides type-safe access to college, campus, group, and schedule data from educational institutions through the OpenScheduleAPI backend.

**Built for OpenScheduleAPI**  
GitHub: [thisishyum/OpenScheduleApi](https://github.com/thisishyum/OpenScheduleApi)

## Features

- **Full OpenScheduleAPI Coverage**: Complete access to colleges, campuses, student groups, and schedules
- **Type-Safe Queries**: Builder pattern for constructing complex queries against OpenScheduleAPI
- **Async/Await Ready**: Built on tokio and reqwest for high-performance async operations
- **Comprehensive Error Handling**: Detailed error types for all OpenScheduleAPI failure scenarios
- **Serde Support**: Full serialization/deserialization support for all OpenScheduleAPI data models
- **Flexible HTTP Client**: Use default client or bring your own configured client
- **Logging Support**: Optional tracing-based logging via feature flag

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
osars = "0.1.0"
```

For logging support, enable the feature:

```toml
[dependencies]
osars = { version = "0.1.0", features = ["logging"] }
```

## Quick Start

```rust
use osars::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with OpenScheduleAPI endpoint
    let client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen/");

    // List all colleges from OpenScheduleAPI
    let colleges = client.colleges().send().await?;
    println!("Available colleges:");
    for college in colleges {
        println!("- {} (ID: {})", college.name, college.college_id);
    }

    // Set default college and explore its structure
    let client = client.with_college(1);
    
    // Get campuses for the college from OpenScheduleAPI
    let campuses = client.campuses()?.send().await?;
    println!("Campuses:");
    for campus in campuses {
        println!("- {} (ID: {})", campus.name, campus.id);
    }

    // Get groups for first campus
    if let Some(first_campus) = campuses.first() {
        let groups = client.groups(first_campus.id).send().await?;
        println!("Groups at {}:", first_campus.name);
        for group in groups {
            println!("- {} (ID: {})", group.name, group.id);
        }

        // Get schedule for first group from OpenScheduleAPI
        if let Some(first_group) = groups.first() {
            let schedule = client.schedule(first_group.id).send().await?;
            println!("Schedule for {}:", first_group.name);
            for day_schedule in schedule {
                println!("Date: {}", day_schedule.date);
                for lesson in day_schedule.lessons {
                    println!("  {}: {} with {} in {}", 
                        lesson.start_time, lesson.title, lesson.teacher, lesson.cabinet);
                }
            }
        }
    }

    Ok(())
}
```

## Core Concepts

### Client
The main entry point that manages OpenScheduleAPI connections and provides query builders.

```rust
// Basic client for OpenScheduleAPI
let client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen/");

// Client with custom HTTP configuration for OpenScheduleAPI
let http_client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build()?;
let client = Client::with_client("https://api.thisishyum.ru/schedule_api/tyumen/", http_client);

// Client with default college for OpenScheduleAPI queries
let client = client.with_college(123);
```

### Query Builders
Chain methods to build complex queries against OpenScheduleAPI:

```rust
// College queries from OpenScheduleAPI
client.colleges().name("техн").send().await?;

// Campus queries from OpenScheduleAPI
client.with_college(1).campuses()?.name("main").send().await?;

// Group queries from OpenScheduleAPI
client.groups(123).name("CS").send().await?;

// Schedule queries from OpenScheduleAPI
client.schedule(456)
    .today()
    .send().await?;

client.schedule(456)
    .week(Week::Current)
    .weekday(Weekday::Monday)
    .send().await?;

client.schedule(456)
    .date("2024-01-15")
    .send().await?;
```

## OpenScheduleAPI Data Models

All models correspond directly to the OpenScheduleAPI response formats.

### College
Represents an educational institution from OpenScheduleAPI.

```rust
pub struct College {
    pub college_id: u32,
    pub name: String,
    pub calls: Vec<Call>,
    pub campuses: Vec<Campus>,
}
```

### Campus
A physical location of a college from OpenScheduleAPI.

```rust
pub struct Campus {
    pub id: u32,
    pub name: String,
    pub college_id: u32,
    pub groups: Vec<Group>,
}
```

### Group
A student group that attends classes together from OpenScheduleAPI.

```rust
pub struct Group {
    pub id: u32,
    pub name: String,
    pub campus_id: u32,
}
```

### Schedule
Daily schedule for a student group from OpenScheduleAPI.

```rust
pub struct Schedule {
    pub group_id: u32,
    pub date: NaiveDate,
    pub lessons: Vec<Lesson>,
}
```

### Lesson
Individual class session from OpenScheduleAPI.

```rust
pub struct Lesson {
    pub title: String,
    pub cabinet: String,
    pub teacher: String,
    pub order: u32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}
```

### Call
Lesson period time slots from OpenScheduleAPI.

```rust
pub struct Call {
    pub call_id: u32,
    pub weekday: u8,
    pub begins: NaiveTime,
    pub ends: NaiveTime,
    pub order: u32,
}
```

## Advanced Usage

### Error Handling for OpenScheduleAPI

```rust
use osars::{Client, Error};

match client.colleges().send().await {
    Ok(colleges) => {
        // Process colleges from OpenScheduleAPI
    }
    Err(Error::Api { status_code, message }) => {
        eprintln!("OpenScheduleAPI error {}: {}", status_code, message);
    }
    Err(Error::Reqwest(e)) => {
        eprintln!("Network error connecting to OpenScheduleAPI: {}", e);
    }
    Err(Error::Serialization(e)) => {
        eprintln!("Data parsing error from OpenScheduleAPI: {}", e);
    }
    Err(Error::Validation(msg)) => {
        eprintln!("Validation error for OpenScheduleAPI query: {}", msg);
    }
    Err(Error::NotFound(msg)) => {
        eprintln!("Resource not found in OpenScheduleAPI: {}", msg);
    }
}
```

### Custom OpenScheduleAPI Endpoints

```rust
// For different OpenScheduleAPI instances
let tyumen_client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen/");
let other_client = Client::new("https://api.example.com/schedule_api/");
```

### Logging with OpenScheduleAPI

Enable the logging feature and initialize:

```rust
use osars::logging;

// Initialize logging
logging::init();

let client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen/");
// All OpenScheduleAPI requests and responses will be logged
```

## Testing

The library includes comprehensive tests against the actual OpenScheduleAPI:

```bash
# Run unit tests
cargo test

# Run integration tests against OpenScheduleAPI
cargo test --test integration_test

# Run documentation tests
cargo test --doc
```

## API Reference

### Client Methods
- `new(base_url)` - Create client for OpenScheduleAPI
- `with_client(base_url, http_client)` - Create client with custom HTTP client
- `with_college(college_id)` - Set default college for queries
- `colleges()` - Query all colleges from OpenScheduleAPI
- `college()` - Query default college
- `campuses()` - Query campuses for default college
- `campus(campus_id)` - Query specific campus
- `groups(campus_id)` - Query groups for campus
- `schedule(group_id)` - Query schedule for group
- `today(group_id)` - Query today's schedule
- `tomorrow(group_id)` - Query tomorrow's schedule

### Query Parameters
- `name(pattern)` - Filter by name pattern
- `date("YYYY-MM-DD")` - Specific date schedule
- `week(Week)` - Week-based schedule (Previous/Current/Next)
- `weekday(Weekday)` - Specific weekday schedule
- `today()` - Today's schedule
- `tomorrow()` - Tomorrow's schedule

## Contributing

This client library is designed to work with the OpenScheduleAPI project. For issues related to the API itself, please refer to the main repository: [thisishyum/OpenScheduleApi](https://github.com/thisishyum/OpenScheduleApi)

## License

This project is licensed under the same license as the OpenScheduleAPI project.

## Related Projects

- [OpenScheduleAPI](https://github.com/thisishyum/OpenScheduleApi) - The main API backend
- OpenScheduleAPI Documentation - Comprehensive API documentation

## Support

For issues with this client library, open an issue in this repository. For issues with the OpenScheduleAPI backend, please refer to the main OpenScheduleAPI repository.
