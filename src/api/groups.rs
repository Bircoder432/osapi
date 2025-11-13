use crate::{Client, Group, error::Result};

pub struct GroupsQuery<'a> {
    client: &'a Client,
    campus_id: u32,
}

impl<'a> GroupsQuery<'a> {
    pub fn new(client: &'a Client, campus_id: u32) -> Self {
        Self { client, campus_id }
    }

    pub async fn send(self) -> Result<Vec<Group>> {
        self.client
            .get_json(&format!("/campuses/{}/groups", self.campus_id))
            .await
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
}
