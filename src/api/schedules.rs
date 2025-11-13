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
        self.validate()?;

        let url = format!(
            "{}/groups/{}/schedules",
            self.client.base_url, self.group_id
        );
        let mut request = self.client.http_client.get(&url);

        request = self.add_query_params(request);

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let body = response.text().await?;
            Err(crate::error::Error::from_response(status.as_u16(), body))
        }
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
