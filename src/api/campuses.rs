use crate::{Campus, Client, error::Result};

pub struct CampusesQuery<'a> {
    client: &'a Client,
    college_id: u32,
}

impl<'a> CampusesQuery<'a> {
    pub fn new(client: &'a Client, college_id: u32) -> Self {
        Self { client, college_id }
    }

    pub async fn send(self) -> Result<Vec<Campus>> {
        self.client
            .get_json(&format!("/colleges/{}/campuses", self.college_id))
            .await
    }

    pub fn campus(self, campus_id: u32) -> CampusQuery<'a> {
        CampusQuery::new(self.client, campus_id)
    }
}

pub struct CampusQuery<'a> {
    client: &'a Client,
    campus_id: u32,
}

impl<'a> CampusQuery<'a> {
    pub fn new(client: &'a Client, campus_id: u32) -> Self {
        Self { client, campus_id }
    }

    pub async fn get(self) -> Result<Campus> {
        self.client
            .get_json(&format!("/campuses/{}", self.campus_id))
            .await
    }

    pub fn groups(self) -> crate::api::groups::GroupsQuery<'a> {
        crate::api::groups::GroupsQuery::new(self.client, self.campus_id)
    }
}
