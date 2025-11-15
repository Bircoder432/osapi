use crate::{Campus, Client, College, api::groups::GroupsQuery, error::Result};
use urlencoding::encode;

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
        let path = if let Some(name) = self.name {
            format!("/colleges?name={}", name)
        } else {
            "/colleges".to_string()
        };
        self.client.get_json(&path).await
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
    name: Option<String>,
}

impl<'a> CampusesQuery<'a> {
    pub fn new(client: &'a Client, college_id: u32) -> Self {
        Self {
            client,
            college_id,
            name: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub async fn send(self) -> Result<Vec<Campus>> {
        let mut url = format!("/colleges/{}/campuses", self.college_id);

        if let Some(name) = self.name {
            url = format!("{}?name={}", url, encode(&name));
        }
        self.client.get_json(&url).await
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
