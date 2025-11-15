use crate::{Campus, Client, error::Result, groups::GroupQuery};

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
        let url = format!("/colleges/{}/campuses", self.college_id);
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
            Err(crate::error::Error::from_response(status.as_u16(), body))
        }
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

    pub fn group(self, group_id: u32) -> GroupQuery<'a> {
        GroupQuery::new(self.client, group_id)
    }
}
