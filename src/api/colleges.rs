use crate::{Campus, Client, College, api::groups::GroupsQuery, error::Result};

pub struct CollegesQuery<'a> {
    client: &'a Client,
}

impl<'a> CollegesQuery<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<Vec<College>> {
        self.client.get_json("/colleges").await
    }

    pub fn college(self, college_id: u32) -> CollegeQuery<'a> {
        CollegeQuery::new(self.client, college_id)
    }
}

pub struct CollegeQuery<'a> {
    client: &'a Client,
    college_id: u32,
}

impl<'a> CollegeQuery<'a> {
    pub fn new(client: &'a Client, college_id: u32) -> Self {
        Self { client, college_id }
    }

    pub async fn get(self) -> Result<College> {
        self.client
            .get_json(&format!("/colleges/{}", self.college_id))
            .await
    }

    pub fn campuses(self) -> CampusesQuery<'a> {
        CampusesQuery::new(self.client, self.college_id)
    }
}

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

    pub fn groups(self) -> GroupsQuery<'a> {
        GroupsQuery::new(self.client, self.campus_id)
    }
}
