use super::ScheduleQuery;
use crate::models::Week;
use crate::{Client, Group, error::Result};

pub struct GroupsQuery<'a> {
    client: &'a Client,
    campus_id: u32,
    name: Option<String>,
}

impl<'a> GroupsQuery<'a> {
    pub fn new(client: &'a Client, campus_id: u32) -> Self {
        Self {
            client,
            campus_id,
            name: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub async fn send(self) -> Result<Vec<Group>> {
        let path = if let Some(name) = self.name {
            format!("/campuses/{}/groups?name={}", self.campus_id, name)
        } else {
            format!("/campuses/{}/groups", self.campus_id)
        };
        self.client.get_json(&path).await
    }

    pub fn group(self, group_id: u32) -> GroupQuery<'a> {
        GroupQuery::new(self.client, group_id)
    }
}

pub struct GroupQuery<'a> {
    client: &'a Client,
    group_id: u32,
}

impl<'a> GroupQuery<'a> {
    pub fn new(client: &'a Client, group_id: u32) -> Self {
        Self { client, group_id }
    }

    pub async fn get(self) -> Result<Group> {
        self.client
            .get_json(&format!("/groups/{}", self.group_id))
            .await
    }

    pub fn schedules(self) -> crate::api::schedules::ScheduleQuery<'a> {
        crate::api::schedules::ScheduleQuery::new(self.client, self.group_id)
    }

    pub fn today(self) -> ScheduleQuery<'a> {
        self.schedules().today()
    }

    pub fn tomorrow(self) -> ScheduleQuery<'a> {
        self.schedules().tomorrow()
    }

    pub fn date(self, date: &str) -> ScheduleQuery<'a> {
        self.schedules().date(date)
    }

    pub fn week(self, week: Week) -> ScheduleQuery<'a> {
        self.schedules().week(week)
    }
}
