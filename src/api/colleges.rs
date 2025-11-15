use std::fmt::format;

use crate::{Campus, Client, College, api::groups::GroupsQuery, error::Result};

pub struct CollegesQuery<'a> {
    client: &'a Client,
    name: Option<String>,
}

impl<'a> CollegesQuery<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client, name: None }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub async fn send(self) -> Result<Vec<College>> {
        let url = "/colleges";
        let mut request = self
            .client
            .http_client
            .get(&format!("{}{}", self.client.base_url, url));
        if let Some(name) = self.name {
            request = request.query(&[("name", name)]);
        }
        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let body = response.text().await?;
            Err(crate::Error::from_response(status.as_u16(), body))
        }
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

    pub fn campus(self, campus_id: u32) -> CampusQuery<'a> {
        CampusQuery::new(self.client, campus_id)
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
    pub fn group(self, group_id: u32) -> crate::api::groups::GroupQuery<'a> {
        crate::api::groups::GroupQuery::new(self.client, group_id)
    }
}
