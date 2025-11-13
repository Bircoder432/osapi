use crate::api::{CollegeQuery, CollegesQuery};
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) base_url: String,
    pub(crate) http_client: reqwest::Client,
    pub(crate) default_college_id: Option<u32>,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
            default_college_id: None,
        }
    }

    pub fn with_client(base_url: &str, http_client: reqwest::Client) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client,
            default_college_id: None,
        }
    }

    pub fn with_college(mut self, college_id: u32) -> Self {
        self.default_college_id = Some(college_id);
        self
    }

    pub fn colleges(&self) -> CollegesQuery {
        CollegesQuery::new(self)
    }

    pub fn college(&self) -> CollegeQuery {
        match self.default_college_id {
            Some(college_id) => CollegeQuery::new(self, college_id),
            None => panic!("No default college set. Use client.with_college() first"),
        }
    }

    pub(crate) async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self.http_client.get(&url).send().await?;

        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let body = response.text().await?;
            Err(crate::error::Error::from_response(status.as_u16(), body))
        }
    }
}
