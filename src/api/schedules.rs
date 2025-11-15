use crate::models::{Day, Week, Weekday};
use crate::{Client, Schedule, error::Result};

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

    pub async fn send(self) -> Result<Vec<Schedule>> {
        let mut params = Vec::new();

        if let Some(date) = self.date {
            params.push(format!("date={}", date));
        }
        if let Some(day) = self.day {
            params.push(format!("day={}", day));
        }
        if let Some(week) = self.week {
            params.push(format!("week={}", week));
        }
        if let Some(weekday) = self.weekday {
            params.push(format!("weekday={}", weekday));
        }

        let query = if params.is_empty() {
            "".to_string()
        } else {
            format!("?{}", params.join("&"))
        };

        let path = format!("/groups/{}/schedules{}", self.group_id, query);
        self.client.get_json(&path).await
    }

    fn validate(&self) -> Result<()> {
        if self.date.is_some()
            && (self.week.is_some() || self.weekday.is_some() || self.day.is_some())
        {
            return Err(crate::error::Error::Validation(
                "parameter 'date' cannot be combined with 'week', 'weekday' or 'day'".to_string(),
            ));
        }

        if self.day.is_some() && (self.week.is_some() || self.weekday.is_some()) {
            return Err(crate::error::Error::Validation(
                "parameter 'day' cannot be combined with 'week' or 'weekday'".to_string(),
            ));
        }

        Ok(())
    }

    fn add_query_params(self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let mut request = request;

        if let Some(date) = self.date {
            request = request.query(&[("date", date)]);
        }

        if let Some(week) = self.week {
            let week_str = match week {
                Week::Previous => "previous",
                Week::Current => "current",
                Week::Next => "next",
            };
            request = request.query(&[("week", week_str)]);
        }

        if let Some(weekday) = self.weekday {
            let weekday_str = match weekday {
                Weekday::Monday => "monday",
                Weekday::Tuesday => "tuesday",
                Weekday::Wednesday => "wednesday",
                Weekday::Thursday => "thursday",
                Weekday::Friday => "friday",
                Weekday::Saturday => "saturday",
                Weekday::Sunday => "sunday",
            };
            request = request.query(&[("weekday", weekday_str)]);
        }

        if let Some(day) = self.day {
            let day_str = match day {
                Day::Today => "today",
                Day::Tomorrow => "tomorrow",
            };
            request = request.query(&[("day", day_str)]);
        }

        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_query_validation() {
        let client = Client::new("https://api.example.com");
        let query = ScheduleQuery::new(&client, 1)
            .date("2023-01-01")
            .week(Week::Current);

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
}
