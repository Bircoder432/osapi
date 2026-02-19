use crate::models::{Day, Week, Weekday};
use crate::{
    Client, Schedule,
    error::{Error, Result},
};

pub struct ScheduleQuery<'a> {
    client: &'a Client,
    group_id: u32,
    date: Option<String>,
    week: Option<Week>,
    weekday: Option<Weekday>,
    day: Option<Day>,
}

impl<'a> ScheduleQuery<'a> {
    pub fn new(client: &'a Client, group_id: u32) -> Self {
        Self {
            client,
            group_id,
            date: None,
            week: None,
            weekday: None,
            day: None,
        }
    }

    pub fn date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    pub fn week(mut self, week: Week) -> Self {
        self.week = Some(week);
        self
    }

    pub fn weekday(mut self, weekday: Weekday) -> Self {
        self.weekday = Some(weekday);
        self
    }

    pub fn today(mut self) -> Self {
        self.day = Some(Day::Today);
        self
    }

    pub fn tomorrow(mut self) -> Self {
        self.day = Some(Day::Tomorrow);
        self
    }

    fn validate(&self) -> Result<()> {
        // Check for incompatible parameter combinations
        if self.date.is_some()
            && (self.week.is_some() || self.weekday.is_some() || self.day.is_some())
        {
            return Err(Error::Validation(
                "parameter 'date' cannot be combined with 'week', 'weekday' or 'day'".to_string(),
            ));
        }

        if self.day.is_some() && (self.week.is_some() || self.weekday.is_some()) {
            return Err(Error::Validation(
                "parameter 'day' cannot be combined with 'week' or 'weekday'".to_string(),
            ));
        }

        Ok(())
    }

    fn build_query_string(&self) -> String {
        let mut params: Vec<(String, String)> = Vec::new();

        if let Some(ref date) = self.date {
            params.push(("date".to_string(), date.clone()));
        }
        if let Some(ref day) = self.day {
            params.push(("day".to_string(), day.to_string()));
        }
        if let Some(ref week) = self.week {
            params.push(("week".to_string(), week.to_string()));
        }
        if let Some(ref weekday) = self.weekday {
            params.push(("weekday".to_string(), weekday.to_string()));
        }

        if params.is_empty() {
            String::new()
        } else {
            let query = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("?{}", query)
        }
    }

    pub async fn send(self) -> Result<Vec<Schedule>> {
        // CRITICAL FIX: validate was defined but never called!
        self.validate()?;

        let query = self.build_query_string();
        let path = format!("/groups/{}/schedules{}", self.group_id, query);
        let url = format!("{}{}", self.client.base_url(), path);

        #[cfg(feature = "logging")]
        tracing::info!("ScheduleQuery URL: {}", url);
        self.client.get_json(&path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, ServerOpts};

    #[test]
    fn test_schedule_query_validation_date_with_week() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1)
            .date("2023-01-01")
            .week(Week::Current);

        let result = query.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("date"));
    }

    #[test]
    fn test_schedule_query_validation_day_with_week() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1).today().week(Week::Current);

        let result = query.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_schedule_query_validation_success() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1).date("2023-01-01");

        let result = query.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_schedule_query_build_empty() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1);

        assert_eq!(query.build_query_string(), "");
    }

    #[test]
    fn test_schedule_query_build_with_date() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1).date("2024-01-15");

        assert_eq!(query.build_query_string(), "?date=2024-01-15");
    }

    #[test]
    fn test_schedule_query_build_complex() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1)
            .week(Week::Current)
            .weekday(Weekday::Monday);

        let qs = query.build_query_string();
        assert!(qs.contains("week=current"));
        assert!(qs.contains("weekday=monday"));
    }

    #[tokio::test]
    async fn test_schedule_query_send_success() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/groups/1/schedules")
            .match_query(mockito::Matcher::UrlEncoded("day".into(), "today".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"groupId": 1, "date": "2024-01-15", "lessons": []}]"#)
            .create_async()
            .await;

        let client = Client::new(&server.url());
        let schedules = client.schedule(1).today().send().await.unwrap();

        assert_eq!(schedules.len(), 1);
        assert_eq!(schedules[0].group_id, 1);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_schedule_query_send_validation_error() {
        let client = Client::new("https://api.example.com");

        // This should fail validation before making any request
        let result = client.schedule(1).date("2024-01-15").today().send().await;

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(_) => (),
            other => panic!("Expected Validation error, got {:?}", other),
        }
    }
}
